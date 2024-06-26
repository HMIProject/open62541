use std::ffi::c_void;

use open62541_sys::{
    UA_Boolean, UA_DataSource, UA_DataValue, UA_NodeId, UA_NumericRange, UA_Server, UA_StatusCode,
};

use crate::{server::NodeContext, ua, DataType};

/// Read callback for [`DataSource`].
#[allow(clippy::module_name_repetitions)]
pub type DataSourceRead = dyn FnMut(&mut DataSourceReadContext) -> DataSourceResult;

/// Write callback for [`DataSource`].
#[allow(clippy::module_name_repetitions)]
pub type DataSourceWrite = dyn FnMut(&mut DataSourceWriteContext) -> DataSourceResult;

/// Data source with callbacks.
///
/// The read and write callbacks implement the operations on the data source when it is added as a
/// variable node via [`Server::add_data_source_variable_node()`].
///
/// [`Server::add_data_source_variable_node()`]: crate::Server::add_data_source_variable_node
pub struct DataSource {
    read: Box<DataSourceRead>,
    write: Option<Box<DataSourceWrite>>,
}

impl DataSource {
    /// Defines read-only data source.
    #[must_use]
    pub fn read_only(
        read: impl FnMut(&mut DataSourceReadContext) -> DataSourceResult + 'static,
    ) -> Self {
        Self {
            read: Box::new(read),
            write: None,
        }
    }

    /// Defines writable data source.
    #[must_use]
    pub fn read_write(
        read: impl FnMut(&mut DataSourceReadContext) -> DataSourceResult + 'static,
        write: impl FnMut(&mut DataSourceWriteContext) -> DataSourceResult + 'static,
    ) -> Self {
        Self {
            read: Box::new(read),
            write: Some(Box::new(write)),
        }
    }

    /// Transforms into raw value.
    ///
    /// # Safety
    ///
    /// The returned [`UA_DataSource`] is only valid for as long as [`NodeContext`] is alive. The
    /// lifetime can be extended by using [`NodeContext::leak()`] to save this value inside the
    /// corresponding server node, to be eventually cleaned up when the node is destroyed.
    pub(crate) unsafe fn into_raw(self) -> (UA_DataSource, NodeContext) {
        unsafe extern "C" fn read_c(
            _server: *mut UA_Server,
            _session_id: *const UA_NodeId,
            _session_context: *mut c_void,
            _node_id: *const UA_NodeId,
            node_context: *mut c_void,
            _include_source_time_stamp: UA_Boolean,
            _range: *const UA_NumericRange,
            _value: *mut UA_DataValue,
        ) -> UA_StatusCode {
            let node_context = unsafe { NodeContext::peek_at(node_context) };
            #[allow(irrefutable_let_patterns)] // We will add more node context types eventually.
            let NodeContext::DataSource(data_source) = node_context
            else {
                // We expect to always find this node context type.
                return ua::StatusCode::BADINTERNALERROR.into_raw();
            };
            let DataSource { read, .. } = data_source;

            // TODO: Prepare callback context.
            let mut context = DataSourceReadContext;

            match read(&mut context) {
                Ok(()) => ua::StatusCode::GOOD,
                Err(status_code) => status_code,
            }
            .into_raw()
        }

        unsafe extern "C" fn write_c(
            _server: *mut UA_Server,
            _session_id: *const UA_NodeId,
            _session_context: *mut c_void,
            _node_id: *const UA_NodeId,
            node_context: *mut c_void,
            _range: *const UA_NumericRange,
            _value: *const UA_DataValue,
        ) -> UA_StatusCode {
            let node_context = unsafe { NodeContext::peek_at(node_context) };
            #[allow(irrefutable_let_patterns)] // We will add more node context types eventually.
            let NodeContext::DataSource(data_source) = node_context
            else {
                // We expect to always find this node context type.
                return ua::StatusCode::BADINTERNALERROR.into_raw();
            };
            let DataSource { write, .. } = data_source;
            let Some(write) = write else {
                return ua::StatusCode::BADWRITENOTSUPPORTED.into_raw();
            };

            // TODO: Prepare callback context.
            let mut context = DataSourceWriteContext;

            match write(&mut context) {
                Ok(()) => ua::StatusCode::GOOD,
                Err(status_code) => status_code,
            }
            .into_raw()
        }

        let data_source = UA_DataSource {
            // The read callback is expected.
            read: Some(read_c),
            // The write callback is optional.
            write: self.write.is_some().then_some(write_c),
        };

        let node_context = NodeContext::DataSource(self);

        (data_source, node_context)
    }
}

// TODO: Add ability to transmit read value back to server.
#[allow(clippy::module_name_repetitions)]
pub struct DataSourceReadContext;

// TODO: Add ability to receive written value from server.
#[allow(clippy::module_name_repetitions)]
pub struct DataSourceWriteContext;

#[allow(clippy::module_name_repetitions)]
pub type DataSourceResult = std::result::Result<(), ua::StatusCode>;
