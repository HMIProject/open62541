use std::{ffi::CString, fmt::Debug, mem::MaybeUninit, ptr};

use open62541_sys::{
    UA_DataType, UA_DataType_clear, UA_DataType_copy, UA_DataType_fromDescription,
    UA_DataType_getStructMember, UA_STATUSCODE_GOOD,
};

use crate::{DataType as _, Error, Result, ua};

/// Wrapper for data type from [`open62541_sys`].
#[repr(transparent)]
pub struct DataType(UA_DataType);

impl DataType {
    pub unsafe fn from_raw(src: UA_DataType) -> Self {
        Self(src)
    }

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

    pub fn from_description(description: ua::ExtensionObject) -> Result<Self> {
        let mut dst = MaybeUninit::<UA_DataType>::uninit();

        let status_code = ua::StatusCode::new(unsafe {
            UA_DataType_fromDescription(dst.as_mut_ptr(), description.as_ptr(), ptr::null())
        });
        Error::verify_good(&status_code)?;

        // SAFETY: We just made sure that the memory region is initialized.
        let dst = unsafe { dst.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        Ok(unsafe { Self::from_raw(dst) })
    }

    pub fn get_struct_member(
        &self,
        value: &ua::ExtensionObject,
        name: &str,
    ) -> Result<ua::Variant> {
        let name = CString::new(name).unwrap();

        let mut out_offset = 0;
        let mut out_member_type = ptr::null();
        let mut out_is_array = false;

        if !unsafe {
            UA_DataType_getStructMember(
                &self.0,
                name.as_ptr(),
                &raw mut out_offset,
                &raw mut out_member_type,
                &raw mut out_is_array,
            )
        } {
            panic!();
        }

        let out_member_type = unsafe { out_member_type.as_ref() }.expect("get member type");
        let mem_size = usize::try_from(out_member_type.memSize()).expect("get memory size");

        // ua::Variant::from_raw(src)

        todo!()
    }

    pub fn decode(&self, value: &ua::ExtensionObject) -> Result<ua::Variant> {
        todo!()
    }

    pub fn decode_raw(&self, value: &ua::ExtensionObject) -> Result<ua::ExtensionObject> {
        let mut value = value.clone();
        unsafe { value.decode(&self.0)? };
        Ok(value)
    }
}

impl Clone for DataType {
    fn clone(&self) -> Self {
        Self::clone_raw(&self.0)
    }
}

impl Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DataType").finish_non_exhaustive()
    }
}

impl Drop for DataType {
    fn drop(&mut self) {
        unsafe { UA_DataType_clear(&mut self.0) };
    }
}
