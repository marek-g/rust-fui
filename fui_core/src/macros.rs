/// A helper macro for `fui` Callbacks that eliminates `Rc` cloning boilerplate.
///
/// This macro handles synchronization (sync/async), ui-provided arguments,
/// and custom manual parameters for viewmodel method binding.
///
/// # Usage Patterns
/// - `cb!(self, method)` -> sync, no args.
/// - `cb!(self, method(10, "abc"))` -> sync, with manual args.
/// - `cb!(self, arg method)` -> sync, with one ui-provided arg.
/// - `cb!(self, arg method(true, _))` -> sync, ui arg (`_`) mixed with manual args.
/// - `cb!(self, async method)` -> async, no args.
/// - `cb!(self, async method(10))` -> async, with manual args.
/// - `cb!(self, async arg method)` -> async, with one ui-provided arg.
/// - `cb!(self, async arg method("search", _))` -> async, ui arg (`_`) mixed with manual args.
///
/// # Requirements
/// - the `$owner` must be an `Rc<T>`.
/// - methods must be available on `T`.
/// - for `arg` variants, ui-provided values and manual clones must implement `Clone`.
#[macro_export]
macro_rules! cb {
    // async + ui arg + manual args
    ($owner:expr, async arg $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_async_rc($owner, |obj, _| {
            let obj = obj.clone();
            let _ = _.clone();
            async move { obj.$method($($args)*).await }
        })
    };

    // async + manual args
    ($owner:expr, async $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_async_rc($owner, |obj| {
            let obj = obj.clone();
            async move { obj.$method($($args)*).await }
        })
    };

    // async + ui arg
    ($owner:expr, async arg $method:ident) => {
        $crate::Callback::new_async_rc($owner, |obj, _| {
            let obj = obj.clone();
            let _ = _.clone();
            async move { obj.$method(_).await }
        })
    };

    // async
    ($owner:expr, async $method:ident) => {
        $crate::Callback::new_async_rc($owner, |obj| {
            let obj = obj.clone();
            async move { obj.$method().await }
        })
    };

    // sync + ui arg + manual args
    ($owner:expr, arg $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_sync_rc($owner, |obj, _| obj.$method($($args)*))
    };

    // sync + manual args
    ($owner:expr, $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_sync_rc($owner, |obj| obj.$method($($args)*))
    };

    // sync + ui arg
    ($owner:expr, arg $method:ident) => {
        $crate::Callback::new_sync_rc($owner, |obj, _| obj.$method(_))
    };

    // sync
    ($owner:expr, $method:ident) => {
        $crate::Callback::new_sync_rc($owner, |obj| obj.$method())
    };
}
