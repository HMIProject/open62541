use std::ffi::c_void;

use open62541_sys::{UA_ExtensionObject_setValueCopy, UA_ExtensionObjectEncoding};

use crate::{DataType, ua};

crate::data_type!(ExtensionObject);

impl ExtensionObject {
    /// Creates extension object from value.
    #[must_use]
    pub fn new<T: DataType>(value: &T) -> Self {
        let mut extension_object = Self::init();
        // We cannot call `UA_ExtensionObject_setValue()`. This would avoid the copy but it would
        // not work on stack-based values because the deallocation happens with `UA_free()`.
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

    /// Gets encoded byte string content.
    #[must_use]
    pub fn encoded_content_bytestring(&self) -> Option<(&ua::NodeId, &ua::ByteString)> {
        match self.0.encoding {
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_BYTESTRING => {}
            _ => return None,
        }

        let encoded_content = unsafe { self.0.content.encoded.as_ref() };

        Some((
            ua::NodeId::raw_ref(&encoded_content.typeId),
            ua::ByteString::raw_ref(&encoded_content.body),
        ))
    }

    /// Gets encoded XML content.
    #[must_use]
    pub fn encoded_content_xml(&self) -> Option<(&ua::NodeId, &ua::String)> {
        match self.0.encoding {
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_ENCODED_XML => {}
            _ => return None,
        }

        let encoded_content = unsafe { self.0.content.encoded.as_ref() };

        Some((
            ua::NodeId::raw_ref(&encoded_content.typeId),
            ua::String::raw_ref(&encoded_content.body),
        ))
    }

    /// Gets decoded content.
    #[must_use]
    pub fn decoded_content<T: DataType>(&self) -> Option<&T> {
        match self.0.encoding {
            UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED
            | UA_ExtensionObjectEncoding::UA_EXTENSIONOBJECT_DECODED_NODELETE => {}
            _ => return None,
        }

        let decoded_content = unsafe { self.0.content.decoded.as_ref() };

        if decoded_content.type_ != T::data_type() {
            return None;
        }

        unsafe { decoded_content.data.cast::<T::Inner>().as_ref() }.map(T::raw_ref)
    }
}
