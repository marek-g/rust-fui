use crate::{
    Application, ChannelDispatcher, DrawingContext, WindowGUIThreadData, WindowId,
    WindowManagerAsync, WindowOptions,
};
use anyhow::Result;
use fui_core::{post_func_current_thread, register_current_thread_dispatcher, ViewModel};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot};

thread_local! {
    pub static APPLICATION_GUI_CONTEXT: RefCell<Option<ApplicationGuiContext >> = RefCell::new(None);
    pub static APPLICATION_VM_CONTEXT: RefCell<Option<ApplicationVmContext >> = RefCell::new(None);
}

///
/// Application data available only from the GUI thread.
///
pub struct ApplicationGuiContext {
    pub drawing_context: Arc<Mutex<DrawingContext>>,
    pub next_window_id: WindowId,
    pub windows: HashMap<WindowId, WindowGUIThreadData>,
    pub func_gui2vm_thread_tx: mpsc::UnboundedSender<Box<dyn 'static + Send + FnOnce()>>,
}

///
/// Application data available only from the VM thread.
///
pub struct ApplicationVmContext {
    pub window_manager: Rc<RefCell<WindowManagerAsync>>,
}

///
/// Application data available only from the VM (View Models) thread.
///
pub struct ApplicationAsync {
    /// GUI thread handle
    gui_thread_join_handle: Option<JoinHandle<()>>,

    /// receiver of the exit event sent from the GUI thread
    gui_thread_exit_rx: oneshot::Receiver<()>,

    /// receiver of closures sent to VM thread from the VM thread
    func_vm2vm_thread_rx: mpsc::UnboundedReceiver<Box<dyn 'static + FnOnce()>>,

    /// receiver of closures sent to VM thread from the GUI or other threads
    func_gui2vm_thread_rx: mpsc::UnboundedReceiver<Box<dyn 'static + Send + FnOnce()>>,
}

impl ApplicationAsync {
    pub async fn new(title: &'static str) -> Result<Self> {
        // channel to send closures VM thread -> VM thread
        let (func_vm2vm_thread_tx, mut func_vm2vm_thread_rx) = mpsc::unbounded_channel();

        // channel to send closures GUI (or any) thread -> VM thread
        let (func_gui2vm_thread_tx, mut func_gui2vm_thread_rx) = mpsc::unbounded_channel();

        // fui_core::Callback uses current thread dispatcher to execute callbacks
        // (callbacks are also used by events, properties, observable collections etc.),
        // it is only needed on the VM thread
        register_current_thread_dispatcher(Box::new(ChannelDispatcher::new(
            func_vm2vm_thread_tx.clone(),
            func_gui2vm_thread_tx.clone(),
        )));

        let (gui_thread_init_tx, gui_thread_init_rx) = oneshot::channel();
        let (gui_thread_exit_tx, gui_thread_exit_rx) = oneshot::channel();

        // start the GUI thread
        let gui_thread_join_handle = std::thread::Builder::new()
            .name("GUI".to_string())
            .spawn(move || {
                let app = fui_system::Application::new(
                    fui_system::ApplicationOptions::new()
                        .with_title(title)
                        .with_opengl_share_contexts(true)
                        .with_opengl_stencil_bits(8),
                )
                .unwrap();

                let drawing_context = Arc::new(Mutex::new(DrawingContext::new().unwrap()));

                APPLICATION_GUI_CONTEXT.with(move |context| {
                    *context.borrow_mut() = Some(ApplicationGuiContext {
                        drawing_context,
                        next_window_id: 1,
                        windows: HashMap::new(),
                        func_gui2vm_thread_tx,
                    })
                });

                // GUI thread reached the initialized state
                gui_thread_init_tx.send(()).unwrap();

                app.message_loop();

                // drop ApplicationGuiContext
                APPLICATION_GUI_CONTEXT.with(move |context| {
                    *context.borrow_mut() = None;
                });

                gui_thread_exit_tx.send(()).unwrap();
            })
            .unwrap();

        // wait for GUI thread to reach the initialized state
        gui_thread_init_rx.await?;

        let text = Rc::new("hello".to_string());

        APPLICATION_VM_CONTEXT.with(move |context| {
            *context.borrow_mut() = Some(ApplicationVmContext {
                window_manager: Rc::new(RefCell::new(WindowManagerAsync::new().unwrap())),
            })
        });

        Ok(Self {
            gui_thread_join_handle: Some(gui_thread_join_handle),
            gui_thread_exit_rx,
            func_vm2vm_thread_rx,
            func_gui2vm_thread_rx,
        })
    }

    pub fn get_window_manager(&self) -> Rc<RefCell<WindowManagerAsync>> {
        APPLICATION_VM_CONTEXT
            .with(move |context| context.borrow().as_ref().unwrap().window_manager.clone())
    }

    pub async fn run(mut self) -> Result<()> {
        let mut exit = false;
        while !exit {
            select! {
                // process all closures sent with dispatcher from the same thread
                f = self.func_vm2vm_thread_rx.recv() => f.unwrap()(),

                // process all closures sent with dispatcher from any thread
                f = self.func_gui2vm_thread_rx.recv() => f.unwrap()(),

                // wait for exit of the gui message loop
                res = &mut self.gui_thread_exit_rx => exit = true,
            }
        }

        // wait for the GUI thread to finish
        if let Some(handle) = self.gui_thread_join_handle.take() {
            handle.join().unwrap();
        }

        Ok(())
    }
}