use crate::{DataType, ua};

crate::data_type!(ViewDescription);
crate::member_accessors!(ViewDescription {
    viewId: &ua::NodeId,
    viewVersion: u32,
    timestamp: i64
});

impl ViewDescription {
    #[must_use]
    pub fn with_view_id(mut self, view_id: &ua::NodeId) -> Self {
        view_id.clone_into_raw(&mut self.0.viewId);
        self
    }

    #[must_use]
    pub const fn with_view_version(mut self, view_version: u32) -> Self {
        self.0.viewVersion = view_version;
        self
    }

    #[must_use]
    pub const fn with_timestamp(mut self, ts: i64) -> Self {
        self.0.timestamp = ts;
        self
    }
}
