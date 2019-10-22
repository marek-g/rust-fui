extern crate winit;

use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, Sender};

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

pub type Job = Box<dyn FnBox + Send + 'static>;

struct DispatcherSource {
    tx: Sender<Job>,
    rx: Receiver<Job>,
    loop_proxy: Option<winit::event_loop::EventLoopProxy<()>>,
}

#[derive(Clone)]
pub struct Dispatcher {
    pub tx: Sender<Job>,
    loop_proxy: Option<winit::event_loop::EventLoopProxy<()>>,
}

impl Dispatcher {
    pub fn setup_events_loop_proxy(loop_proxy: winit::event_loop::EventLoopProxy<()>) {
        CURRENT_THREAD_DISPATCHER.with(|x| {
            let mut borrowed = x.borrow_mut();
            if let Some(ref mut dispatcher_source) = *borrowed {
                dispatcher_source.loop_proxy = Some(loop_proxy);
                return;
            }
            let (tx, rx) = channel();
            let dispatcher_source = DispatcherSource {
                tx,
                rx,
                loop_proxy: Some(loop_proxy),
            };
            *borrowed = Some(dispatcher_source);
        })
    }

    pub fn for_current_thread() -> Dispatcher {
        CURRENT_THREAD_DISPATCHER.with(|x| {
            let borrowed = x.borrow();
            if let Some(ref dispatcher_source) = *borrowed {
                let tx = dispatcher_source.tx.clone();
                return Dispatcher { tx, loop_proxy: dispatcher_source.loop_proxy.clone() };
            }
            panic!("Call Dispatcher::setup_events_loop_proxy() before first Dispatcher::for_current_thread()!");
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
        if let Some(ref loop_proxy) = self.loop_proxy {
            loop_proxy.send_event(()).unwrap();
        }
    }
}
