use crate::JoinHandle;
use std::future::Future;

///
/// Spawn task on the same thread and return handle that will abort it on drop.
///
pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    JoinHandle::new(tokio::task::spawn_local(future))
}

///
/// Spawn task on the same thread.
///
pub fn spawn_local_and_forget<F>(future: F)
where
    F: Future + 'static,
{
    tokio::task::spawn_local(future);
}
