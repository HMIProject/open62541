use std::{ffi::c_void, ptr};

use open62541_sys::{
    UA_DataType, UA_ExtensionObject_clear, UA_ExtensionObject_setValue,
    UA_ExtensionObject_setValueCopy, UA_ExtensionObjectEncoding, UA_decodeBinary,
};

use crate::{DataType, Error, Result, ua};

// SAFETY: This must only hold primitive values, i.e. all directly and indirectly used `UA_DataType`
// must be statically allocated. Otherwise, we'd risk use-after-free.
crate::data_type!(ExtensionObject);

impl ExtensionObject {
    /// Creates extension object from value.
    #[must_use]
    pub fn new<T: DataType>(value: &T) -> Self {
        let mut extension_object = Self::init();

        // We cannot call `UA_ExtensionObject_setValue()`. This would avoid the copy but it would
        // not work on stack-based values because deallocation always happens with `UA_free()`.
        unsafe {
            UA_ExtensionObject_setValueCopy(
                extension_object.as_mut_ptr(),
                // SAFETY: `UA_ExtensionObject_setValueCopy()` expects `*mut c_void` but does not
                // actually mutate the value, it only calls `UA_copy()` internally.
                value.as_ptr().cast::<c_void>().cast_mut(),
                T::data_type(),
            );
        }

        extension_object
    }

    /// Gets encoded type ID.
    #[must_use]
    pub fn encoded_type_id(&self) -> Option<&ua::NodeId> {
        if !matches!(
            self.0.encoding,
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_NOBODY
                | UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_BYTESTRING
                | UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_XML
        ) {
            return None;
        }

        // SAFETY: Encoding indicates encoded object, union variant is valid.
        let encoded_content = unsafe { self.0.content.encoded.as_ref() };

        Some(&ua::NodeId::raw_ref(&encoded_content.typeId))
    }

    /// Gets encoded byte string content.
    #[must_use]
    pub fn encoded_content_bytestring(&self) -> Option<(&ua::NodeId, &ua::ByteString)> {
        if !matches!(
            self.0.encoding,
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_BYTESTRING
        ) {
            return None;
        }

        // SAFETY: Encoding indicates encoded object, union variant is valid.
        let encoded_content = unsafe { self.0.content.encoded.as_ref() };

        Some((
            ua::NodeId::raw_ref(&encoded_content.typeId),
            ua::ByteString::raw_ref(&encoded_content.body),
        ))
    }

    /// Gets encoded XML content.
    #[must_use]
    pub fn encoded_content_xml(&self) -> Option<(&ua::NodeId, &ua::String)> {
        if !matches!(
            self.0.encoding,
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_XML
        ) {
            return None;
        }

        // SAFETY: Encoding indicates encoded object, union variant is valid.
        let encoded_content = unsafe { self.0.content.encoded.as_ref() };

        Some((
            ua::NodeId::raw_ref(&encoded_content.typeId),
            ua::String::raw_ref(&encoded_content.body),
        ))
    }

    /// Gets decoded content.
    #[must_use]
    pub fn decoded_content<T: DataType>(&self) -> Option<&T> {
        if !matches!(
            self.0.encoding,
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED
                | UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED_NODELETE
        ) {
            return None;
        }

        let decoded_content = unsafe { self.0.content.decoded.as_ref() };

        // This matches the implementation of `UA_ExtensionObject_hasDecodedType()`.
        if decoded_content.data.is_null() || decoded_content.type_ != T::data_type() {
            return None;
        }

        // SAFETY: Pointer is valid and points to decoded data of the expected type.
        unsafe { decoded_content.data.cast::<T::Inner>().as_ref() }.map(T::raw_ref)
    }

    pub(crate) fn raw_decoded_content_mut(
        &mut self,
        data_type: *const UA_DataType,
    ) -> Option<*mut c_void> {
        if !matches!(
            self.0.encoding,
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED
                | UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED_NODELETE
        ) {
            return None;
        }

        let decoded_content = unsafe { self.0.content.decoded.as_ref() };

        // This matches the implementation of `UA_ExtensionObject_hasDecodedType()`.
        if decoded_content.data.is_null() || decoded_content.type_ != data_type {
            return None;
        }

        Some(decoded_content.data)
    }

    /// Decodes extension object.
    ///
    /// This turns an extension object with encoded representation (transport encoding, i.e., binary
    /// or XML) into one with decoded representation (for direct memory access). When the object has
    /// been decoded already, this is a no-op.
    ///
    /// This succeeds only when the extension object has the given type, encoded or already decoded.
    /// In case of an error, the original object is left as-is.
    ///
    /// # Safety
    ///
    /// `data_type` must outlive `self` (this pointer becomes part of the extension object's decoded
    /// representation).
    pub(crate) unsafe fn decode(&mut self, data_type: *const UA_DataType) -> Result<()> {
        if matches!(
            self.0.encoding,
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED
                | UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED_NODELETE
        ) {
            let decoded_content = unsafe { self.0.content.decoded.as_ref() };

            // This matches the implementation of `UA_ExtensionObject_hasDecodedType()`.
            if decoded_content.data.is_null() || decoded_content.type_ != data_type {
                return Err(Error::Internal("unexpected extension object data type"));
            }

            return Ok(());
        }

        // TODO: Add decoding of XML encoding with `UA_decodeXml()`.
        if !matches!(
            self.0.encoding,
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_BYTESTRING
        ) {
            return Err(Error::Internal("unsupported encoding of extension object"));
        }

        // SAFETY: Encoding indicates encoded object, union variant is valid.
        let encoded_content = unsafe { self.0.content.encoded.as_ref() };
        let type_id = ua::NodeId::raw_ref(&encoded_content.typeId);
        let body = ua::ByteString::raw_ref(&encoded_content.body);

        // PANIC: The given pointer must be valid.
        let data_type_ref = unsafe { data_type.as_ref() }.expect("valid data type");
        if type_id != ua::NodeId::raw_ref(&data_type_ref.typeId) {
            return Err(Error::Internal("unexpected extension object data type ID"));
        }

        // PANIC: `usize` must hold the given `UA_UInt32` value.
        let mem_size = usize::try_from(data_type_ref.memSize()).expect("get memory size");
        let mut data: Vec<u8> = Vec::with_capacity(mem_size);

        // This puts data into the heap-allocated vector that might include pointers or referenced
        // data structures. After this has succeded, we must not drop `data` or risk memory leaks.
        let status_code = ua::StatusCode::new(unsafe {
            UA_decodeBinary(
                body.as_ptr(),
                data.as_mut_ptr().cast::<c_void>(),
                data_type,
                ptr::null_mut(),
            )
        });
        Error::verify_good(&status_code)?;

        // Use helper functions that clean up `encoded_content` (owned type ID, owned byte string)
        // and replaces it with pointers to the given arguments. We leak heap-allocated `data`, to
        // be recaptured by `self.0`. This is also where `data_type` becomes part of `self`.
        unsafe {
            UA_ExtensionObject_clear(&mut self.0);
            UA_ExtensionObject_setValue(
                &mut self.0,
                data.leak().as_mut_ptr().cast::<c_void>(),
                data_type,
            );
        }

        Ok(())
    }
}
