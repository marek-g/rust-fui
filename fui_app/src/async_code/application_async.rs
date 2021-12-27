use crate::Application;
use anyhow::Result;
use std::thread;
use std::thread::JoinHandle;

pub struct ApplicationAsync {
    thread_join_handle: Option<JoinHandle<()>>,
}

impl ApplicationAsync {
    pub fn new(title: &'static str) -> Result<Self> {
        let thread_join_handle = std::thread::Builder::new()
            .name("GUI".to_string())
            .spawn(move || {
                let mut app = Application::new(title).unwrap();

                /*app.add_window(
                    WindowOptions::new()
                        .with_title("Example: async")
                        .with_size(800, 600),
                    MainViewModel::new(),
                )
                .unwrap();*/

                println!("Running qt thread: {:?}", thread::current().id());

                app.run().unwrap();
                //app.run_async();
            })
            .unwrap();

        Ok(Self {
            thread_join_handle: Some(thread_join_handle),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        if let Some(handle) = self.thread_join_handle.take() {
            handle.join().unwrap();
        }
        Ok(())
    }
}
