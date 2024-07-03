use std::{
    ffi::c_void,
    panic::{catch_unwind, AssertUnwindSafe},
};

use open62541_sys::{
    UA_Boolean, UA_DataSource, UA_DataValue, UA_NodeId, UA_NumericRange, UA_Server, UA_StatusCode,
};

use crate::{server::NodeContext, ua, DataType};

/// Data source with callbacks.
///
/// The read and write callbacks implement the operations on the variable when it is added via
/// [`Server::add_data_source_variable_node()`].
///
/// [`Server::add_data_source_variable_node()`]: crate::Server::add_data_source_variable_node
pub trait DataSource {
    /// Reads from variable.
    ///
    /// This is called when a client wants to read the value from the variable.
    ///
    /// # Errors
    ///
    /// This should return an appropriate error when the read is not possible. The underlying status
    /// code is forwarded to the client.
    fn read(&mut self, context: &mut DataSourceReadContext) -> DataSourceResult;

    /// Writes to variable.
    ///
    /// This is called when a client wants to write the value to the variable.
    ///
    /// # Errors
    ///
    /// This should return an appropriate error when the write is not possible. The underlying
    /// status code is forwarded to the client.
    fn write(&mut self, context: &mut DataSourceWriteContext) -> DataSourceResult;
}

#[allow(clippy::type_complexity)]
struct CallbackDataSource {
    read: Box<dyn FnMut(&mut DataSourceReadContext) -> DataSourceResult>,
    write: Option<Box<dyn FnMut(&mut DataSourceWriteContext) -> DataSourceResult>>,
}

impl CallbackDataSource {
    #[must_use]
    fn read_only(
        read: impl FnMut(&mut DataSourceReadContext) -> DataSourceResult + 'static,
    ) -> Self {
        Self {
            read: Box::new(read),
            write: None,
        }
    }

    /// Defines writable data source.
    #[must_use]
    fn read_write(
        read: impl FnMut(&mut DataSourceReadContext) -> DataSourceResult + 'static,
        write: impl FnMut(&mut DataSourceWriteContext) -> DataSourceResult + 'static,
    ) -> Self {
        Self {
            read: Box::new(read),
            write: Some(Box::new(write)),
        }
    }
}

impl DataSource for CallbackDataSource {
    fn read(&mut self, context: &mut DataSourceReadContext) -> DataSourceResult {
        (self.read)(context)
    }

    fn write(&mut self, context: &mut DataSourceWriteContext) -> DataSourceResult {
        let Some(write) = &mut self.write else {
            return Err(ua::StatusCode::BADWRITENOTSUPPORTED);
        };
        write(context)
    }
}

/// Defines read-only data source.
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn read_only_data_source(
    read: impl FnMut(&mut DataSourceReadContext) -> DataSourceResult + 'static,
) -> impl DataSource {
    CallbackDataSource::read_only(read)
}

/// Defines writable data source.
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn read_write_data_source(
    read: impl FnMut(&mut DataSourceReadContext) -> DataSourceResult + 'static,
    write: impl FnMut(&mut DataSourceWriteContext) -> DataSourceResult + 'static,
) -> impl DataSource {
    CallbackDataSource::read_write(read, write)
}

/// Transforms into raw value.
///
/// # Safety
///
/// The returned [`UA_DataSource`] is only valid for as long as [`NodeContext`] is alive. The
/// lifetime can be extended by using [`NodeContext::leak()`] to save this value inside the
/// corresponding server node, to be eventually cleaned up when the node is destroyed.
pub(crate) unsafe fn wrap_data_source(
    data_source: impl DataSource + 'static,
) -> (UA_DataSource, NodeContext) {
    unsafe extern "C" fn read_c(
        _server: *mut UA_Server,
        _session_id: *const UA_NodeId,
        _session_context: *mut c_void,
        _node_id: *const UA_NodeId,
        node_context: *mut c_void,
        _include_source_time_stamp: UA_Boolean,
        _range: *const UA_NumericRange,
        value: *mut UA_DataValue,
    ) -> UA_StatusCode {
        let node_context = unsafe { NodeContext::peek_at(node_context) };
        #[allow(irrefutable_let_patterns)] // We will add more node context types eventually.
        let NodeContext::DataSource(data_source) = node_context
        else {
            // We expect to always find this node context type.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };

        let mut data_source = AssertUnwindSafe(data_source);
        let mut context = DataSourceReadContext { value };

        match catch_unwind(move || data_source.read(&mut context)) {
            Ok(Ok(())) => ua::StatusCode::GOOD,
            Ok(Err(status_code)) => status_code,
            Err(err) => {
                log::error!("Read callback in data source panicked: {err:?}");
                ua::StatusCode::BADINTERNALERROR
            }
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
        value: *const UA_DataValue,
    ) -> UA_StatusCode {
        let node_context = unsafe { NodeContext::peek_at(node_context) };
        #[allow(irrefutable_let_patterns)] // We will add more node context types eventually.
        let NodeContext::DataSource(data_source) = node_context
        else {
            // We expect to always find this node context type.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };

        let mut data_source = AssertUnwindSafe(data_source);
        let mut context = DataSourceWriteContext { value };

        match catch_unwind(move || data_source.write(&mut context)) {
            Ok(Ok(())) => ua::StatusCode::GOOD,
            Ok(Err(status_code)) => status_code,
            Err(err) => {
                log::error!("Write callback in data source panicked: {err:?}");
                ua::StatusCode::BADINTERNALERROR
            }
        }
        .into_raw()
    }

    let raw_data_source = UA_DataSource {
        // The read callback is expected.
        read: Some(read_c),
        // The write callback is optional.
        write: Some(write_c),
    };

    let node_context = NodeContext::DataSource(Box::new(data_source));

    (raw_data_source, node_context)
}

/// Context when [`DataSource`] is being read from.
#[allow(clippy::module_name_repetitions)]
pub struct DataSourceReadContext {
    value: *mut UA_DataValue,
}

impl DataSourceReadContext {
    /// Sets value.
    ///
    /// This sets the value to report back to the client that is reading from this [`DataSource`].
    pub fn set_value(&mut self, value: ua::DataValue) {
        if let Some(target) = unsafe { self.value.as_mut() } {
            value.move_into_raw(target);
        } else {
            panic!("value should be set in DataSourceReadContext");
        }
    }

    /// Sets variant.
    ///
    /// This is a shortcut for setting the value to report back to the client. See [`set_value()`].
    ///
    /// [`set_value()`]: Self::set_value
    pub fn set_variant(&mut self, variant: ua::Variant) {
        self.set_value(ua::DataValue::new(variant));
    }
}

/// Context when [`DataSource`] is being written to.
#[allow(clippy::module_name_repetitions)]
pub struct DataSourceWriteContext {
    value: *const UA_DataValue,
}

impl DataSourceWriteContext {
    /// Gets value.
    ///
    /// This returns the value received from the client that is writing to this [`DataSource`].
    pub fn value(&self) -> &ua::DataValue {
        if let Some(source) = unsafe { self.value.as_ref() } {
            ua::DataValue::raw_ref(source)
        } else {
            panic!("value should be set in DataSourceWriteContext");
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub type DataSourceResult = std::result::Result<(), ua::StatusCode>;
