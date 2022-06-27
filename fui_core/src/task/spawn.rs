use crate::JoinHandle;
use std::future::Future;

///
/// Spawn local task and return handle that will abort it on drop.
///
pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    JoinHandle::new(tokio::task::spawn_local(future))
}
