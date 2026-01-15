use std::{
    ffi::{CStr, CString},
    fmt::Debug,
    mem::{self, MaybeUninit},
    pin::Pin,
    ptr, slice,
};

use open62541_sys::{
    UA_DataType, UA_DataType_clear, UA_DataType_copy, UA_DataType_fromDescription,
    UA_DataType_getStructMember, UA_STATUSCODE_GOOD,
};

use crate::{DataType as _, Error, Result, ua};

/// Wrapper for data type from [`open62541_sys`].
///
/// The actual data type is held on the heap to preserve pointer addresses after move.
///
/// # Safety
///
/// Instances of this type may be used _by address_, e.g., in [`ua::DataType`], [`ua::Variant`], and
/// [`ua::ExtensionObject`]. In this case, the data type instance must live at least as long as that
/// value.
#[repr(transparent)]
pub struct DataType(UA_DataType);

// SAFETY: Like other data types, `UA_DataType` may be sent to another thread.
unsafe impl Send for DataType {}

// SAFETY: Like other data types, `UA_DataType` may be accessed by other threads.
unsafe impl Sync for DataType {}

impl DataType {
    /// Creates wrapper by taking ownership of data type.
    ///
    /// # Safety
    ///
    /// Ownership of the data type passes to `Self`. The data type must not have been referenced yet
    /// by address in other values (as moving it into `Self` changes the address).
    unsafe fn from_raw(src: UA_DataType) -> Self {
        Self(src)
    }

    /// Creates wrapper reference from value.
    #[must_use]
    pub(crate) fn raw_ref(src: &UA_DataType) -> &Self {
        let src: *const UA_DataType = src;
        // This transmutes between the inner type and `Self` through `cast()`. This is valid because
        // of `#[repr(transparent)]`.
        let ptr = src.cast::<Self>();
        // SAFETY: `#[repr(transparent)]` allows us to transmute between `Self` and the inner type.
        let ptr = unsafe { ptr.as_ref() };
        // SAFETY: Pointer is valid (non-zero) because it comes from a reference.
        unsafe { ptr.unwrap_unchecked() }
    }

    // For now, we deliberately do not implement `Clone` to prevent subtle mistakes (e.g., when data
    // types get reused without updating all references to them before dropping the original).
    #[expect(dead_code, reason = "unused for now")]
    fn clone_raw(src: &UA_DataType) -> Self {
        let src: *const UA_DataType = src;
        let mut dst = MaybeUninit::<UA_DataType>::uninit();

        let result = unsafe { UA_DataType_copy(src, dst.as_mut_ptr()) };
        assert_eq!(result, UA_STATUSCODE_GOOD, "should have copied value");

        // SAFETY: We just made sure that the memory region is initialized.
        let dst = unsafe { dst.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(dst) }
    }

    /// Returns const pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_ptr(&self) -> *const UA_DataType {
        let this: *const Self = self;
        // This transmutes between `Self` and the inner type through `cast()`. Types that implement
        // `DataType` guarantee that we can transmute between them and their inner type, so this is
        // okay.
        this.cast::<UA_DataType>()
    }

    pub(crate) fn from_description(
        description: ua::ExtensionObject,
        custom_types: Option<Pin<&ua::DataTypeArray>>,
    ) -> Result<Self> {
        let mut dst = MaybeUninit::<UA_DataType>::uninit();

        let status_code = ua::StatusCode::new(unsafe {
            UA_DataType_fromDescription(
                dst.as_mut_ptr(),
                description.as_ptr(),
                custom_types
                    .as_deref()
                    .map_or(ptr::null(), ua::DataTypeArray::as_ptr),
            )
        });
        Error::verify_good(&status_code)?;

        // SAFETY: We just made sure that the memory region is initialized.
        let dst = unsafe { dst.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        Ok(unsafe { Self::from_raw(dst) })
    }

    #[must_use]
    pub fn type_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.typeId)
    }

    #[must_use]
    pub fn binary_encoding_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.binaryEncodingId)
    }

    #[must_use]
    pub fn xml_encoding_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.xmlEncodingId)
    }

    pub fn get_struct_member(
        &self,
        value: &mut ua::ExtensionObject,
        name: &str,
    ) -> Result<ua::Variant> {
        let name = CString::new(name).unwrap();

        let mut out_offset = 0;
        let mut out_member_type = ptr::null();
        let mut out_is_array = false;

        if !unsafe {
            UA_DataType_getStructMember(
                &raw const self.0,
                name.as_ptr(),
                &raw mut out_offset,
                &raw mut out_member_type,
                &raw mut out_is_array,
            )
        } {
            return Err(Error::Internal("unknown struct member"));
        }

        let out_member_type = unsafe { out_member_type.as_ref() }.expect("get member type");
        let member_size = usize::try_from(out_member_type.memSize()).expect("get member size");

        // // FIXME: Unwrap. Unsafe decode.
        // unsafe { value.decode(&raw const self.0).unwrap() };
        // let Some(data) = value.raw_decoded_content_mut(&raw const self.0) else {
        //     panic!();
        // };

        // let member_data =
        //     unsafe { slice::from_raw_parts_mut(data.cast::<u8>().add(out_offset), member_size) };

        // Ok(ua::Var::scalar(out_member_type, member_data))

        todo!()
    }

    /// Gives up ownership and returns value.
    #[must_use]
    pub(crate) fn into_raw(self) -> UA_DataType {
        // SAFETY: Type `#[repr(transparent)]`.
        unsafe { mem::transmute(self) }
    }
}

impl Drop for DataType {
    fn drop(&mut self) {
        // Remove all dynamically allocated data structures within the data type.
        unsafe { UA_DataType_clear(&mut self.0) };
    }
}

impl Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // SAFETY: We have a pointer to a valid C string.
        let type_name = unsafe { CStr::from_ptr(self.0.typeName) };

        f.debug_struct("DataType")
            .field("typeName", &type_name.to_string_lossy())
            .field("typeId", ua::NodeId::raw_ref(&self.0.typeId))
            .finish_non_exhaustive()
    }
}
