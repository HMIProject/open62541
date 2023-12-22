use std::{cmp, ffi::CString, hash, str};

use open62541_sys::{
    UA_NodeIdType, UA_NodeId_equal, UA_NodeId_hash, UA_NodeId_order, UA_NodeId_parse, UA_Order,
    UA_NODEID_NUMERIC, UA_NODEID_STRING_ALLOC,
};

use crate::{data_type::DataType, ua, Error};

crate::data_type!(NodeId);

impl NodeId {
    /// Creates node ID for numeric identifier.
    #[must_use]
    pub fn numeric(ns_index: u16, numeric: u32) -> Self {
        let inner = unsafe { UA_NODEID_NUMERIC(ns_index, numeric) };
        debug_assert_eq!(
            inner.identifierType,
            UA_NodeIdType::UA_NODEIDTYPE_NUMERIC,
            "new node ID should have numeric type"
        );

        Self(inner)
    }

    /// Creates node ID for string identifier.
    ///
    /// # Panics
    ///
    /// The string identifier must not contain any NUL bytes.
    #[must_use]
    pub fn string(ns_index: u16, string: &str) -> Self {
        let string = CString::new(string).expect("node ID string does not contain NUL bytes");

        // Technically, string allocation may fail but `UA_NODEID_STRING_ALLOC` doesn't tell us that
        // when it happens. Instead, we end up with a well-defined node ID that has an empty string.
        let inner = unsafe { UA_NODEID_STRING_ALLOC(ns_index, string.as_ptr()) };
        debug_assert_eq!(
            inner.identifierType,
            UA_NodeIdType::UA_NODEIDTYPE_STRING,
            "new node ID should have string type"
        );

        // SAFETY: We have checked that we have this enum variant.
        let identifier = unsafe { inner.identifier.string.as_ref() };
        if !string.is_empty() && (identifier.data.is_null() || identifier.length == 0) {
            // We don't want to leak memory on top.
            debug_assert!(identifier.data.is_null());
            panic!("node ID string should have been allocated");
        }

        Self(inner)
    }

    #[must_use]
    pub fn identifier_type(&self) -> ua::NodeIdType {
        ua::NodeIdType::new(self.0.identifierType.clone())
    }
}

impl cmp::PartialEq for NodeId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { UA_NodeId_equal(self.as_ptr(), other.as_ptr()) }
    }
}

impl cmp::Eq for NodeId {}

impl cmp::PartialOrd for NodeId {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(<Self as cmp::Ord>::cmp(self, other))
    }
}

impl cmp::Ord for NodeId {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let order = unsafe { UA_NodeId_order(self.as_ptr(), other.as_ptr()) };

        match order {
            UA_Order::UA_ORDER_LESS => cmp::Ordering::Less,
            UA_Order::UA_ORDER_EQ => cmp::Ordering::Equal,
            UA_Order::UA_ORDER_MORE => cmp::Ordering::Greater,
            _ => panic!("should return valid order"),
        }
    }
}

impl hash::Hash for NodeId {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let hash = unsafe { UA_NodeId_hash(self.as_ptr()) };

        state.write_u32(hash);
    }
}

impl str::FromStr for NodeId {
    type Err = Error;

    /// ```
    /// use open62541::ua;
    ///
    /// // Valid node IDs can be parsed.
    /// let node_id: ua::NodeId = "ns=0;i=2258".parse().expect("should be valid node ID");
    ///
    /// // Node IDs are normalized (note that `ns=0` has been dropped).
    /// assert_eq!(node_id.to_string(), "i=2258");
    ///
    /// // Parsing node IDs can fail.
    /// "LoremIpsum".parse::<ua::NodeId>().expect_err("should be invalid node ID");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut node_id = NodeId::init();

        let status_code = ua::StatusCode::new({
            let str: ua::String = s.parse()?;
            let str = str.into_raw();
            unsafe { UA_NodeId_parse(node_id.as_mut_ptr(), str) }
        });
        Error::verify_good(status_code)?;

        Ok(node_id)
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::fmt;

    use super::NodeId;

    impl serde::Serialize for NodeId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.collect_str(self)
        }
    }

    impl<'de> serde::Deserialize<'de> for NodeId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(NodeIdVisitor)
        }
    }

    struct NodeIdVisitor;

    impl<'de> serde::de::Visitor<'de> for NodeIdVisitor {
        type Value = NodeId;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an OPC UA node ID")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse()
                .map_err(|_| serde::de::Error::custom("invalid node ID"))
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::ua;

        #[test]
        fn json_serialization() {
            let node_id: ua::NodeId =
                serde_json::from_str(r#""ns=0;i=2258""#).expect("should deserialize node ID");

            assert_eq!(
                serde_json::to_string(&node_id).expect("should serialize node ID"),
                r#""i=2258""#
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str;

    use crate::ua;

    #[test]
    fn string_representation() {
        // We explicitly derive `FromStr` and `ToString`. This is part of the public interface.
        //
        let node_id =
            <ua::NodeId as str::FromStr>::from_str("ns=0;i=2258").expect("should be valid node ID");

        assert_eq!(<ua::NodeId as ToString>::to_string(&node_id), "i=2258");

        // Usually, parsing is done via `parse()` however which is implemented for `FromStr` target.
        //
        let _node_id: ua::NodeId = "ns=0;i=2258".parse().expect("should be valid node ID");
    }
}
