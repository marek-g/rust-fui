use crate::{DrawingContext, WindowGUIThreadData, WindowId, WindowVMThreadData};
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tokio::select;
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
    // translates WindowId to window data - needed for events
    pub windows: HashMap<WindowId, Weak<WindowVMThreadData>>,
}

///
/// Application data available only from the VM (View Models) thread.
///
pub struct Application {
    /// GUI thread handle
    gui_thread_join_handle: Option<JoinHandle<()>>,

    /// handle to message loop of VM thread
    message_loop_handle: tokio::task::JoinHandle<()>,
}

impl Application {
    pub async fn new(title: &'static str) -> Result<Self> {
        // channel to send closures GUI (or any) thread -> VM thread
        let (func_gui2vm_thread_tx, mut func_gui2vm_thread_rx) = mpsc::unbounded_channel();

        let (gui_thread_init_tx, gui_thread_init_rx) = oneshot::channel();
        let (gui_thread_exit_tx, mut gui_thread_exit_rx) = oneshot::channel();

        // start the GUI thread
        let gui_thread_join_handle = std::thread::Builder::new()
            .name("GUI".to_string())
            .spawn(move || {
                let app = windowing_qt::Application::new(
                    windowing_qt::ApplicationOptions::new()
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

        APPLICATION_VM_CONTEXT.with(move |context| {
            *context.borrow_mut() = Some(ApplicationVmContext {
                windows: HashMap::new(),
            })
        });

        // spawn task to handle closures sent from other threads
        //
        // it is spawned from Application::new() and not from run(),
        // because some operations like painting newly created windows
        // require it to be running to enable communication between
        // GUI and VM threads without deadlocks
        let message_loop_handle = tokio::task::spawn_local(async move {
            let mut exit = false;
            while !exit {
                select! {
                    // process all closures sent with from other threads
                    f = func_gui2vm_thread_rx.recv() => if let Some(f) = f { f() },

                    // wait for exit of the gui message loop
                    _res = &mut gui_thread_exit_rx => exit = true,
                }
            }
        });

        Ok(Self {
            gui_thread_join_handle: Some(gui_thread_join_handle),
            message_loop_handle,
        })
    }

    pub fn exit() {
        windowing_qt::Application::exit(0);
    }

    pub async fn run(mut self) -> Result<()> {
        self.message_loop_handle.await?;

        // wait for the GUI thread to finish
        if let Some(handle) = self.gui_thread_join_handle.take() {
            handle.join().unwrap();
        }

        Ok(())
    }
}
