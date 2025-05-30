use std::{
    ffi::c_void,
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::NonNull,
};

use open62541_sys::{
    UA_Boolean, UA_DataSource, UA_DataValue, UA_NodeId, UA_NumericRange, UA_Server, UA_StatusCode,
};
use thiserror::Error;

use crate::{server::NodeContext, ua, DataType as _, Error};

/// Result from [`DataSource`] operations.
///
/// On success, the operations return `Ok(())`. The actual value is transmitted through the
/// `context` argument. See [`DataSource::read()`] and [`DataSource::write()`] for details.
pub type DataSourceResult = Result<(), DataSourceError>;

#[derive(Debug, Error)]
pub enum DataSourceError {
    #[error("{0}")]
    StatusCode(ua::StatusCode),

    #[error(transparent)]
    Error(#[from] Error),
}

impl DataSourceError {
    #[must_use]
    pub fn from_status_code(status_code: ua::StatusCode) -> Self {
        // Any good error would be misleading.
        Self::StatusCode(if status_code.is_good() {
            ua::StatusCode::BADINTERNALERROR
        } else {
            status_code
        })
    }

    pub(crate) fn into_status_code(self) -> ua::StatusCode {
        match self {
            DataSourceError::StatusCode(status_code) => status_code,
            DataSourceError::Error(err) => err.status_code(),
        }
    }
}

/// Data source with callbacks.
///
/// The `read` and `write` callbacks implement the operations on the variable when it is added via
/// [`Server::add_data_source_variable_node()`].
///
/// [`Server::add_data_source_variable_node()`]: crate::Server::add_data_source_variable_node
pub trait DataSource {
    /// Reads from variable.
    ///
    /// This is called when a client wants to read the value from the variable. The value is
    /// expected to be returned through the `context` argument. See
    /// [`DataSourceReadContext::set_value()`] for details.
    ///
    /// # Errors
    ///
    /// This should return an appropriate error when the read is not possible. The underlying status
    /// code is forwarded to the client.
    // TODO: Check if we can guarantee `&mut self`.
    fn read(&mut self, context: &mut DataSourceReadContext) -> DataSourceResult;

    /// Writes to variable.
    ///
    /// This is called when a client wants to write the value to the variable. The value is
    /// transmitted through the `context` argument. See [`DataSourceWriteContext::value()`] for
    /// details.
    ///
    /// If this method is not implemented, [`ua::StatusCode::BADNOTSUPPORTED`] is returned to the
    /// client.
    ///
    /// # Errors
    ///
    /// This should return an appropriate error when the write is not possible. The underlying
    /// status code is forwarded to the client.
    // TODO: Check if we can guarantee `&mut self`.
    #[expect(unused_variables, reason = "unused in default implementation")]
    fn write(&mut self, context: &mut DataSourceWriteContext) -> DataSourceResult {
        Err(DataSourceError::from_status_code(
            ua::StatusCode::BADNOTSUPPORTED,
        ))
    }
}

/// Context when [`DataSource`] is being read from.
#[derive(Debug)]
pub struct DataSourceReadContext {
    /// Outgoing value to be read.
    ///
    /// This is a mutable cell where the read callback puts the data to be returned to the client.
    value_target: NonNull<UA_DataValue>,
}

impl DataSourceReadContext {
    /// Creates context for `read` callback.
    fn new(value: *mut UA_DataValue) -> Option<Self> {
        Some(Self {
            value_target: NonNull::new(value)?,
        })
    }

    /// Gets mutable reference to value.
    ///
    /// This allows setting the value to report back to the client that is reading from this
    /// [`DataSource`].
    #[must_use]
    pub fn value_mut(&mut self) -> &mut ua::DataValue {
        let value_source = unsafe { self.value_target.as_mut() };
        ua::DataValue::raw_mut(value_source)
    }

    /// Sets value.
    ///
    /// This sets the value to report back to the client that is reading from this [`DataSource`].
    pub fn set_value(&mut self, value: ua::DataValue) {
        *self.value_mut() = value;
    }

    /// Sets variant.
    ///
    /// This is a shortcut for setting the value to report back to the client. See [`set_value()`].
    ///
    /// [`set_value()`]: Self::set_value
    pub fn set_variant(&mut self, variant: ua::Variant) {
        *self.value_mut() = ua::DataValue::new(variant);
    }
}

/// Context when [`DataSource`] is being written to.
#[derive(Debug)]
pub struct DataSourceWriteContext {
    /// Incoming value to be written.
    ///
    /// This is an immutable (const) cell where the write callback receives the data to be written
    /// by the client.
    value_source: NonNull<UA_DataValue>,
}

impl DataSourceWriteContext {
    /// Creates context for `write` callback.
    fn new(value: *const UA_DataValue) -> Option<Self> {
        Some(Self {
            // SAFETY: `NonNull` implicitly expects a `*mut` but we take care to never mutate the
            // target.
            value_source: NonNull::new(value.cast_mut())?,
        })
    }

    /// Gets value.
    ///
    /// This returns the value received from the client that is writing to this [`DataSource`].
    #[must_use]
    pub fn value(&self) -> &ua::DataValue {
        let value_source = unsafe { self.value_source.as_ref() };
        ua::DataValue::raw_ref(value_source)
    }
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
        let NodeContext::DataSource(data_source) = node_context else {
            // We expect to always find this node context type.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };

        let Some(mut context) = DataSourceReadContext::new(value) else {
            // Creating context for callback should always succeed.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };
        let mut data_source = AssertUnwindSafe(data_source);

        let status_code = match catch_unwind(move || data_source.read(&mut context)) {
            Ok(Ok(())) => ua::StatusCode::GOOD,
            Ok(Err(err)) => err.into_status_code(),
            Err(err) => {
                log::error!("Read callback in data source panicked: {err:?}");
                ua::StatusCode::BADINTERNALERROR
            }
        };

        status_code.into_raw()
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
        let NodeContext::DataSource(data_source) = node_context else {
            // We expect to always find this node context type.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };

        let Some(mut context) = DataSourceWriteContext::new(value) else {
            // Creating context for callback should always succeed.
            return ua::StatusCode::BADINTERNALERROR.into_raw();
        };
        let mut data_source = AssertUnwindSafe(data_source);

        let status_code = match catch_unwind(move || data_source.write(&mut context)) {
            Ok(Ok(())) => ua::StatusCode::GOOD,
            Ok(Err(err)) => err.into_status_code(),
            Err(err) => {
                log::error!("Write callback in data source panicked: {err:?}");
                ua::StatusCode::BADINTERNALERROR
            }
        };

        status_code.into_raw()
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
