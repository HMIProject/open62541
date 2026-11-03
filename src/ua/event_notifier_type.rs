crate::data_type::data_type_baseline!(
    #[expect(
        clippy::derived_hash_with_manual_eq,
        reason = "open62541 numeric equality impl satisfies `Hash`s identity invariant"
    )]
    #[derive(Clone, Copy, Hash)]
    EventNotifierType
);

crate::enum_bitmask!(
    enum EventNotifierType : u8 {
        /// Indicates if it can be used to subscribe to events.
        SubscribeToEvents,
        /// Indicates if the history of the events is readable.
        HistoryRead,
        /// Indicates if the history of the events is writeable.
        HistoryWrite,
    }
);
