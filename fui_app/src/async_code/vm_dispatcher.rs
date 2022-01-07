pub struct VMDispatcher();

impl fui_core::Dispatcher for VMDispatcher {
    fn post_func_same_thread(&mut self, func: Box<dyn FnOnce()>) {
        unsafe {
            /*tokio::spawn(async move {
                func();
            });*/
            // safe to call, because dispatcher is placed on local thread
            //self.0.post_func_same_thread(func);
        }
    }

    fn post_func_any_thread(&mut self, func: Box<dyn FnOnce() + Send>) {
        tokio::spawn(async move {
            func();
        });
    }
}
