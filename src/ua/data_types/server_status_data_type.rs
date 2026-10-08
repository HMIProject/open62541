use open62541_sys::UA_ServerState;

use crate::ua;

crate::data_type!(ServerStatusDataType);
crate::member_accessors!(ServerStatusDataType {
    buildInfo: &ua::BuildInfo,
    #[enum(UA_ServerState)]
    state: ua::ServerState,
    #[from_inner]
    startTime: ua::DateTime,
    #[from_inner]
    currentTime: ua::DateTime,
    secondsTillShutdown: u32,
    shutdownReason: &ua::LocalizedText
});
