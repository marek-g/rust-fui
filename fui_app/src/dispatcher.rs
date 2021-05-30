pub struct Dispatcher(pub fui_system::Dispatcher);

impl fui_core::Dispatcher for Dispatcher {
    fn post_func(&mut self, func: Box<dyn FnOnce()>) {
        self.0.post_func(func);
    }
}
