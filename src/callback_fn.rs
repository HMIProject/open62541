mod callback_mut;
mod callback_once;

#[cfg_attr(
    not(feature = "experimental-monitored-item-callback"),
    expect(
        unreachable_pub,
        reason = "Only needs to be public when this features is enabled."
    )
)]
pub use self::callback_mut::CallbackMut;

// TODO: Reduce visibility to pub(crate).
pub use self::callback_once::CallbackOnce;
