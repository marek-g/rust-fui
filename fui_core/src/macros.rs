/// A helper macro for `fui` Callbacks that eliminates `Rc` cloning boilerplate
/// and provides ergonomic syntax for binding ViewModel methods to UI events.
///
/// # Usage Patterns
///
/// ## Sync callbacks
///
/// ```rust,ignore
/// // Method with no arguments - UI argument is ignored
/// cb!(self, decrease)
/// // Expands to: Callback::new_sync_rc(self, |obj, _| obj.decrease())
///
/// // Method that uses the UI-provided argument
/// cb!(self, handle_click(_))
/// // Expands to: Callback::new_sync_rc(self, |obj, _| obj.handle_click(_))
///
/// // Method with manual arguments only
/// cb!(self, set_value(42))
/// // Expands to: Callback::new_sync_rc(self, |obj, _| obj.set_value(42))
///
/// // Method with mix of manual and UI arguments
/// cb!(self, process("start", _, 100))
/// // Expands to: Callback::new_sync_rc(self, |obj, _| obj.process("start", _, 100))
/// ```
///
/// ## Async callbacks
///
/// ```rust,ignore
/// // Async method with no arguments
/// cb!(self, async file_open)
/// // Expands to: Callback::new_async_rc(self, |obj, _| async move { obj.file_open().await })
///
/// // Async method using UI argument
/// cb!(self, async handle_input(_))
/// // Expands to: Callback::new_async_rc(self, |obj, _| async move { obj.handle_input(_).await })
///
/// // Async with manual + UI arguments
/// cb!(self, async save_data("backup", _))
/// // Expands to: Callback::new_async_rc(self, |obj, _| async move { obj.save_data("backup", _).await })
/// ```
///
/// # Requirements
/// - `$owner` must be an `Rc<T>` where `T` implements the method being called.
/// - Methods taking `self: Rc<Self>` work directly; for `&self` methods, ensure proper binding.
/// - The placeholder `_` represents the UI-provided argument of type `A` from `Callback<A>`.
/// - If a method requires `Clone` on the UI argument, call `_.clone()` explicitly in the argument list.
///
/// # Examples
///
/// ```rust,ignore
/// use fui_core::Callback;
/// use std::rc::Rc;
///
/// struct ViewModel;
/// impl ViewModel {
///     fn increment(self: Rc<Self>) { /* ... */ }
///     fn on_click(self: Rc<Self>, event: MouseEvent) { /* ... */ }
///     async fn save(self: Rc<Self>, path: &str, event: MouseEvent) { /* ... */ }
/// }
///
/// let vm = Rc::new(ViewModel);
///
/// // Simple sync callback, event ignored
/// let c1: Callback<()> = cb!(vm, increment);
///
/// // Sync callback using the UI event
/// let c2: Callback<MouseEvent> = cb!(vm, on_click(_));
///
/// // Async callback with manual + UI argument
/// let c3: Callback<MouseEvent> = cb!(vm, async save("/data.txt", _));
/// ```
///
/// # Notes
/// - This macro **always** uses `new_sync_rc` / `new_async_rc` internally because it is
///   designed for binding ViewModel methods that take `Rc<Self>`.
/// - All generated closures have the signature `|obj, ui_arg|`. Use `_` to ignore `ui_arg`
///   when not needed.
/// - No automatic cloning of `ui_arg` is performed. If your method requires a `Clone`,
///   write `_.clone()` explicitly in the argument list.
/// - For async methods, the closure is wrapped in `async move` to properly capture ownership.
#[macro_export]
macro_rules! cb {
    // === ASYNC with manual arguments (UI arg may be used via `_` in the list) ===
    ($owner:expr, async $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_async_rc($owner, move |obj, _| {
            let obj = obj.clone();
            async move { obj.$method($($args)*).await }
        })
    };

    // === ASYNC without manual arguments (UI arg ignored) ===
    ($owner:expr, async $method:ident) => {
        $crate::Callback::new_async_rc($owner, move |obj, _| {
            let obj = obj.clone();
            async move { obj.$method().await }
        })
    };

    // === SYNC with manual arguments (UI arg may be used via `_` in the list) ===
    ($owner:expr, $method:ident ( $($args:tt)* )) => {
        $crate::Callback::new_sync_rc($owner, move |obj, _| {
            let obj = obj.clone();
            obj.$method($($args)*)
        })
    };

    // === SYNC without manual arguments (UI arg ignored) ===
    ($owner:expr, $method:ident) => {
        $crate::Callback::new_sync_rc($owner, move |obj, _| {
            let obj = obj.clone();
            obj.$method()
        })
    };
}
