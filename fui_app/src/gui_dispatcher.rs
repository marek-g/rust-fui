pub struct GUIDispatcher(pub fui_system::Dispatcher);

impl fui_core::Dispatcher for GUIDispatcher {
    fn post_func_same_thread(&mut self, func: Box<dyn FnOnce()>) {
        unsafe {
            // safe to call, because dispatcher is placed on local thread
            self.0.post_func_same_thread(func);
        }
    }

    fn post_func_any_thread(&mut self, func: Box<dyn FnOnce() + Send>) {
        self.0.post_func_any_thread(func);
    }
}
