use std::{fmt, str};

use open62541_sys::{
    UA_EXPANDEDNODEID_NODEID, UA_EXPANDEDNODEID_NUMERIC, UA_ExpandedNodeId_parse,
    UA_ExpandedNodeId_print, UA_NodeIdType,
};

use crate::{DataType as _, Error, ua};

crate::data_type!(ExpandedNodeId);

impl ExpandedNodeId {
    /// Creates expanded node ID.
    ///
    /// Constructs a new instance from parts by taking ownership.
    ///
    /// # Example
    ///
    /// ```
    /// use open62541::ua;
    ///
    /// let node_id = ua::NodeId::numeric(1, 234);
    /// let namespace_uri = ua::String::new("http://www.example.com").unwrap();
    /// let server_index = 99;
    ///
    /// let expanded_node_id = ua::ExpandedNodeId::new(
    ///     node_id.clone(),
    ///     namespace_uri.clone(),
    ///     server_index);
    ///
    /// assert_eq!(expanded_node_id.node_id(), &node_id);
    /// assert_eq!(expanded_node_id.namespace_uri(), &namespace_uri);
    /// assert_eq!(expanded_node_id.server_index(), server_index);
    /// ```
    #[must_use]
    pub fn new(node_id: ua::NodeId, namespace_uri: ua::String, server_index: u32) -> Self {
        let mut new = Self::from(node_id);
        debug_assert!(new.namespace_uri().is_null());
        // This passes ownership into the created expanded node ID.
        new.0.namespaceUri = namespace_uri.into_raw();
        new.0.serverIndex = server_index;
        new
    }

    /// Creates numeric expanded node ID.
    #[must_use]
    pub fn numeric(ns_index: u16, numeric: u32) -> Self {
        let inner = unsafe { UA_EXPANDEDNODEID_NUMERIC(ns_index, numeric) };
        debug_assert_eq!(
            inner.nodeId.identifierType,
            UA_NodeIdType::UA_NODEIDTYPE_NUMERIC,
            "new node ID should have numeric type"
        );

        Self(inner)
    }

    #[must_use]
    pub fn node_id(&self) -> &ua::NodeId {
        ua::NodeId::raw_ref(&self.0.nodeId)
    }

    #[must_use]
    pub fn namespace_uri(&self) -> &ua::String {
        ua::String::raw_ref(&self.0.namespaceUri)
    }

    #[must_use]
    pub const fn server_index(&self) -> u32 {
        self.0.serverIndex
    }
}

impl From<ua::NodeId> for ExpandedNodeId {
    fn from(node_id: ua::NodeId) -> Self {
        // This passes ownership into the created expanded node ID.
        Self(unsafe { UA_EXPANDEDNODEID_NODEID(node_id.into_raw()) })
    }
}

impl str::FromStr for ExpandedNodeId {
    type Err = Error;

    /// Parses expanded node ID from string.
    ///
    /// # Examples
    ///
    /// ```
    /// use open62541::ua;
    ///
    /// // Valid expanded node IDs can be parsed.
    /// let node_xid: ua::ExpandedNodeId = "nsu=urn:example.com:my-server;s=myVariable".parse().expect("should be valid expanded node ID");
    ///
    /// assert_eq!(node_xid.to_string(), "nsu=urn:example.com:my-server;s=myVariable");
    ///
    /// // Parsing expanded node IDs can fail.
    /// "LoremIpsum".parse::<ua::ExpandedNodeId>().expect_err("should be invalid expanded node ID");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut node_id = ExpandedNodeId::init();

        let status_code = ua::StatusCode::new({
            let str = ua::String::new(s)?;
            // SAFETY: `UA_ExpandedNodeId_parse()` expects the string passed by value but does not take
            // ownership.
            let str = unsafe { ua::String::to_raw_copy(&str) };
            unsafe { UA_ExpandedNodeId_parse(node_id.as_mut_ptr(), str) }
        });
        Error::verify_good(&status_code)?;

        Ok(node_id)
    }
}

impl fmt::Display for ExpandedNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = ua::String::init();

        let status_code = &ua::StatusCode::new({
            // This mirrors the behavior of `UA_ExpandedNodeId_parse()` above.
            unsafe { UA_ExpandedNodeId_print(self.as_ptr(), output.as_mut_ptr()) }
        });
        Error::verify_good(status_code).map_err(|_| fmt::Error)?;

        output.as_str().unwrap_or("").fmt(f)
    }
}
