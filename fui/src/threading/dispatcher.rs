use std::cell::RefCell;
use std::sync::{ Arc, Mutex, mpsc::{ Sender, Receiver, channel } };

thread_local! {
    static CURRENT_THREAD_DISPATCHER: RefCell<Option<DispatcherSource>> = RefCell::new(None);
}

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce() + Send + 'static> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub type Job = Box<FnBox + Send + 'static>;

struct DispatcherSource {
    tx: Sender<Job>,
    rx: Receiver<Job>,
}

#[derive(Clone)]
pub struct Dispatcher {
    pub tx: Sender<Job>,
}

impl Dispatcher {
    pub fn for_current_thread() -> Dispatcher {
        CURRENT_THREAD_DISPATCHER.with(|x| {
            let mut borrowed = x.borrow_mut();
            if let Some(ref dispatcher_source) = *borrowed {
                let tx = dispatcher_source.tx.clone();
                return Dispatcher { tx };
            }
            let (tx, rx) = channel();
            let tx_clone = tx.clone();
            let dispatcher_source = DispatcherSource { tx, rx };
            *borrowed = Some(dispatcher_source);
            Dispatcher { tx: tx_clone }
        })
    }

    pub fn execute_all_in_queue() {
        CURRENT_THREAD_DISPATCHER.with(|x| {
            if let Some(ref dispatcher_source) = *x.borrow() {
                while let Ok(f) = dispatcher_source.rx.try_recv() {
                    f.call_box();
                }
            }
        })
    }

    pub fn send_async<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.tx.send(Box::new(f)).unwrap();
    }
}
