use std::{mem, ptr::NonNull};

use open62541_sys::{
    UA_KeyValueMap, UA_KeyValueMap_contains, UA_KeyValueMap_delete, UA_KeyValueMap_get,
    UA_KeyValueMap_new, UA_KeyValueMap_remove, UA_KeyValueMap_set,
};

use crate::{ua, DataType, Error};

/// Wrapper for [`UA_KeyValueMap`] from [`open62541_sys`].
#[derive(Debug)]
pub struct KeyValueMap(NonNull<UA_KeyValueMap>);

impl KeyValueMap {
    /// Creates wrapper initialized with defaults.
    #[must_use]
    pub(crate) fn init() -> Self {
        // PANIC: The only possible errors here are out-of-memory.
        let key_value_map =
            NonNull::new(unsafe { UA_KeyValueMap_new() }).expect("should create key value map");

        Self(key_value_map)
    }

    /// Creates new map from existing elements.
    ///
    /// This copies over the elements from the given slice. The map will own the copies, and clean
    /// up when it is dropped. The original elements in the slice are left untouched.
    ///
    /// # Panics
    ///
    /// Enough memory must be available to allocate map.
    #[must_use]
    pub fn from_slice(slice: &[(&ua::QualifiedName, &ua::Variant)]) -> Self {
        let mut key_value_map = Self::init();

        for (key, value) in slice {
            key_value_map.set(key, value);
        }

        key_value_map
    }

    /// Checks whether map has key.
    #[must_use]
    pub fn contains(&self, key: &ua::QualifiedName) -> bool {
        unsafe {
            UA_KeyValueMap_contains(
                self.as_ptr(),
                // SAFETY: `UA_KeyValueMap_contains()` reads the key but does not take ownership.
                DataType::to_raw_copy(key),
            )
        }
    }

    /// Gets key's value from map.
    #[must_use]
    pub fn get(&self, key: &ua::QualifiedName) -> Option<&ua::Variant> {
        let variant = unsafe {
            UA_KeyValueMap_get(
                self.as_ptr(),
                // SAFETY: `UA_KeyValueMap_get()` reads the key but does not take ownership.
                DataType::to_raw_copy(key),
            )
        };

        // SAFETY: Pointer is either null or a valid reference (to a variant value with the lifetime
        // of `self`).
        unsafe { variant.as_ref() }.map(ua::Variant::raw_ref)
    }

    /// Sets key's value in map.
    ///
    /// This replaces a previously set value for this key.
    ///
    /// # Panics
    ///
    /// Enough memory must be available to add value to map.
    pub fn set(&mut self, key: &ua::QualifiedName, value: &ua::Variant) {
        let status_code = ua::StatusCode::new(unsafe {
            UA_KeyValueMap_set(
                self.as_mut_ptr(),
                // SAFETY: `UA_KeyValueMap_set()` reads the key but does not take ownership. In both
                // cases (key already exists or is inserted for the first time), an internal copy is
                // made before inserting into the data structure.
                DataType::to_raw_copy(key),
                value.as_ptr(),
            )
        });

        Error::verify_good(&status_code).expect("should add value");
    }

    /// Removes key's value from map.
    ///
    /// This returns `true` if the key was removed, and `false` if the key did not exist.
    pub fn remove(&mut self, key: &ua::QualifiedName) -> bool {
        let status_code = ua::StatusCode::new(unsafe {
            UA_KeyValueMap_remove(
                self.as_mut_ptr(),
                // SAFETY: `UA_KeyValueMap_remove()` reads the key but does not take ownership.
                DataType::to_raw_copy(key),
            )
        });

        if status_code == ua::StatusCode::GOOD {
            true
        } else if status_code == ua::StatusCode::BADNOTFOUND {
            false
        } else {
            // PANIC: Function returns no other status codes (the other code `BADINVALIDARGUMENT` is
            // only returned when we pass in a null-pointer for the map).
            unreachable!("failed to remove key: {status_code}");
        }
    }

    /// Gives up ownership and returns value.
    #[allow(dead_code)] // This is unused for now.
    pub(crate) const fn into_inner(self) -> *mut UA_KeyValueMap {
        let key_value_map = self.0.as_ptr();
        // Make sure that `drop()` is not called anymore.
        mem::forget(self);
        key_value_map
    }

    /// Returns const pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) const unsafe fn as_ptr(&self) -> *const UA_KeyValueMap {
        self.0.as_ptr()
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_KeyValueMap {
        self.0.as_ptr()
    }
}

impl Drop for KeyValueMap {
    fn drop(&mut self) {
        unsafe { UA_KeyValueMap_delete(self.0.as_mut()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations() {
        // Map can be created from initial values.
        let mut key_value_map = KeyValueMap::from_slice(&[
            (
                &ua::QualifiedName::new(1, "lorem"),
                &ua::Variant::scalar(ua::UInt16::new(123)),
            ),
            (
                &ua::QualifiedName::new(2, "ipsum"),
                &ua::Variant::scalar(ua::String::new("dolor").expect("create string")),
            ),
        ]);

        // Presence of values can be checked.
        assert!(key_value_map.contains(&ua::QualifiedName::new(1, "lorem")));
        assert!(key_value_map.contains(&ua::QualifiedName::new(2, "ipsum")));
        assert!(!key_value_map.contains(&ua::QualifiedName::new(3, "ipsum")));

        // Existing value can be returned.
        assert_eq!(
            key_value_map
                .get(&ua::QualifiedName::new(1, "lorem"))
                .expect("value `1:lorem` should exist")
                .as_scalar(),
            Some(&ua::UInt16::new(123))
        );

        // Non-existent values are handled.
        assert_eq!(key_value_map.get(&ua::QualifiedName::new(1, "dolor")), None);
        assert!(!key_value_map.contains(&ua::QualifiedName::new(1, "dolor")));

        // New value can be added, other values are untouched.
        key_value_map.set(
            &ua::QualifiedName::new(3, "ipsum"),
            &ua::Variant::scalar(ua::Float::new(9.87)),
        );
        assert_eq!(
            key_value_map
                .get(&ua::QualifiedName::new(3, "ipsum"))
                .expect("value `3:ipsum` should exist")
                .as_scalar(),
            Some(&ua::Float::new(9.87))
        );
        assert!(key_value_map.contains(&ua::QualifiedName::new(3, "ipsum")));
        assert_eq!(
            key_value_map
                .get(&ua::QualifiedName::new(1, "lorem"))
                .expect("value `1:lorem` should exist")
                .as_scalar(),
            Some(&ua::UInt16::new(123))
        );
        assert!(key_value_map.contains(&ua::QualifiedName::new(2, "ipsum")));
    }

    // TODO: Enable when <https://github.com/open62541/open62541/issues/6905> has been fixed.
    #[allow(dead_code)]
    #[cfg_attr(any(), test)]
    fn remove_key() {
        let mut key_value_map = KeyValueMap::from_slice(&[
            (
                &ua::QualifiedName::new(1, "lorem"),
                &ua::Variant::scalar(ua::UInt16::new(123)),
            ),
            (
                &ua::QualifiedName::new(2, "ipsum"),
                &ua::Variant::scalar(ua::String::new("dolor").expect("create string")),
            ),
        ]);

        // Value can be removed, other values are untouched.
        let was_removed = key_value_map.remove(&ua::QualifiedName::new(1, "lorem"));
        assert!(was_removed);
        assert_eq!(key_value_map.get(&ua::QualifiedName::new(1, "lorem")), None);
        assert!(!key_value_map.contains(&ua::QualifiedName::new(1, "lorem")));
        assert_eq!(
            key_value_map
                .get(&ua::QualifiedName::new(2, "ipsum"))
                .expect("value `2:ipsum` should exist")
                .as_scalar(),
            Some(&ua::String::new("dolor").expect("create string"))
        );
        assert!(key_value_map.contains(&ua::QualifiedName::new(2, "ipsum")));

        // Removing non-existent value is handled.
        let was_removed = key_value_map.remove(&ua::QualifiedName::new(1, "lorem"));
        assert!(!was_removed);

        // Value can be set again.
        key_value_map.set(
            &ua::QualifiedName::new(1, "lorem"),
            &ua::Variant::scalar(ua::Float::new(1.23)),
        );
        assert_eq!(
            key_value_map
                .get(&ua::QualifiedName::new(1, "lorem"))
                .expect("value `1:lorem` should exist")
                .as_scalar(),
            Some(&ua::Float::new(1.23))
        );

        // Existing value can be overwritten.
        key_value_map.set(
            &ua::QualifiedName::new(1, "lorem"),
            &ua::Variant::scalar(ua::Double::new(3.21)),
        );
        assert_eq!(
            key_value_map
                .get(&ua::QualifiedName::new(1, "lorem"))
                .expect("value `1:lorem` should exist")
                .as_scalar(),
            Some(&ua::Double::new(3.21))
        );
    }
}
