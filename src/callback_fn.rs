mod callback_mut;
mod callback_once;

pub(crate) use self::callback_mut::CallbackMut;
// TODO: Reduce visibility to pub(crate).
pub use self::callback_once::CallbackOnce;
