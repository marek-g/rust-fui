/// A convenience macro to create synchronized or asynchronous callbacks
/// linked to an owner (typically `Rc` or `Arc`).
///
/// This macro automates the process of cloning the owner into the closure
/// and wrapping the execution in an async block when needed.
///
/// # Variations
///
/// * **With UI Argument**: `cb!(owner, |v| method(v))`
///   Use this when you need to access the data passed by the callback (e.g., a button click value).
///   The identifier between `| |` becomes the name of the argument within the method call.
///
/// * **With Manual Arguments**: `cb!(owner, method("static", 42))`
///   Use this when you want to call a method with specific, predefined arguments,
///   ignoring the data provided by the callback itself.
///
/// * **No Arguments**: `cb!(owner, method)`
///   A shorthand for calling a method that takes no parameters (other than `&self`).
///
/// * **Async Support**:
///   Prefix the method name with the `async` keyword to use `new_async_rc`.
///   The macro will automatically wrap the call in an `async move` block.
///
/// # Examples
///
/// ```rust
/// // 1. Using the callback's value
/// let c1 = cb!(self, |msg| handle_message(msg));
///
/// // 2. Passing static data (ignoring callback value)
/// let c2 = cb!(self, async save_to_disk("/tmp/log.txt"));
///
/// // 3. Simple notification
/// let c3 = cb!(self, refresh_ui);
/// ```
#[macro_export]
macro_rules! cb {
	// async with explicit argument name: cb!(obj, |v| async method(v))
    ($owner:expr, |$arg_name:ident| async $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_async_rc($owner, move |obj, $arg_name| {
            let obj = obj.clone();
            async move { obj.$method($($args)*).await }
        })
    };

    // async with manual arguments
    ($owner:expr, async $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_async_rc($owner, move |obj, arg| {
            let obj = obj.clone();
			let arg = arg;
            async move { obj.$method($($args)*).await }
        })
    };

    // async without arguments
    ($owner:expr, async $method:ident) => {
        $crate::Callback::new_async_rc($owner, move |obj, _| {
            let obj = obj.clone();
            async move { obj.$method().await }
        })
    };

	// sync with explicit argument name: cb!(obj, |v| method(v))
    ($owner:expr, |$arg_name:ident| $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_sync_rc($owner, move |obj, $arg_name| {
            let obj = obj.clone();
            obj.$method($($args)*)
        })
    };

    // sync with manual arguments
    ($owner:expr, $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_sync_rc($owner, move |obj, arg| {
            let obj = obj.clone();
			let arg = arg;
            obj.$method($($args)*)
        })
    };

    // sync without arguments
    ($owner:expr, $method:ident) => {
        $crate::Callback::new_sync_rc($owner, move |obj, _| {
            let obj = obj.clone();
            obj.$method()
        })
    };
}
