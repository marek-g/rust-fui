///
/// JoinHandle will abort the task when dropped.
///
pub struct JoinHandle<T> {
    handle: tokio::task::JoinHandle<T>,
}

impl<T> JoinHandle<T> {
    pub fn new(handle: tokio::task::JoinHandle<T>) -> Self {
        JoinHandle { handle }
    }
}

impl<T> Drop for JoinHandle<T> {
    fn drop(&mut self) {
        self.handle.abort();
    }
}
