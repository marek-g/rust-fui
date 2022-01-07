use tokio::sync::mpsc;

pub struct ChannelDispatcher {
    func_same_thread_tx: mpsc::UnboundedSender<Box<dyn 'static + FnOnce()>>,
    func_any_thread_tx: mpsc::UnboundedSender<Box<dyn 'static + Send + FnOnce()>>,
}

impl ChannelDispatcher {
    pub fn new(
        same_thread_tx: mpsc::UnboundedSender<Box<dyn 'static + FnOnce()>>,
        any_thread_tx: mpsc::UnboundedSender<Box<dyn 'static + Send + FnOnce()>>,
    ) -> Self {
        Self {
            func_same_thread_tx: same_thread_tx,
            func_any_thread_tx: any_thread_tx,
        }
    }
}

impl fui_core::Dispatcher for ChannelDispatcher {
    fn post_func_same_thread(&mut self, func: Box<dyn FnOnce()>) {
        self.func_same_thread_tx.send(func);
    }

    fn post_func_any_thread(&mut self, func: Box<dyn FnOnce() + Send>) {
        self.func_any_thread_tx.send(func);
    }
}
