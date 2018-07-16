extern crate failure;
extern crate time;
extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
extern crate fui;
extern crate std;

pub type Result<T> = std::result::Result<T, failure::Error>;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{ Arc, Mutex };
use std::sync::mpsc::*;
use self::gst::prelude::*;
use fui::*;
use pipeline_factory;

pub struct Player {
    pub texture: PlayerTexture,
    pipeline: Option<self::gst::Pipeline>,
    dispatcher: Arc<Mutex<Dispatcher>>,
    receiver: Option<Receiver<Vec<u8>>>,
}

impl Player {
    pub fn new(drawing_context: Rc<RefCell<DrawingContext>>) -> Self {
        gst::init().unwrap();

        Player {
            texture: PlayerTexture::new(drawing_context),
            pipeline: None,
            dispatcher: Arc::new(Mutex::new(Dispatcher::for_current_thread())),
            receiver: None,
        }
    }

    pub fn open(&mut self) {
        println!("Main thread id: {:?}", std::thread::current().id());

        let (sender, receiver) = channel();
        self.receiver = Some(receiver);
        let sender = Arc::new(Mutex::new(sender));

        // Create the elements
        //let (pipeline, video_app_sink) = pipeline_factory::create_pipeline_videotest();
        //self.texture.set_size(320, 240);
        let (pipeline, video_app_sink) = pipeline_factory::create_pipeline_url("http://ftp.nluug.nl/pub/graphics/blender/demo/movies/Sintel.2010.720p.mkv");
        self.texture.set_size(1280, 544);

        let dispatcher_clone = self.dispatcher.clone();
        video_app_sink.set_callbacks(gst_app::AppSinkCallbacks::new()
            .new_sample(move |app_sink| {
                let timespec = time::get_time();
                let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
                println!("New sample thread id: {:?}, time: {:?}", std::thread::current().id(), mills);

                let sample = match app_sink.pull_sample() {
                    None => return gst::FlowReturn::Eos,
                    Some(sample) => sample,
                };

                let caps = sample.get_caps().unwrap();
                let s = caps.get_structure(0).unwrap();
                let width: i32 = s.get("width").unwrap();
                let height: i32 = s.get("height").unwrap();
                let buffer = sample.get_buffer().unwrap();
                let map = buffer.map_readable().unwrap();
                let data = map.as_slice();

                //print!("AppSink: New sample ({}x{}, size: {})\n", width, height, data.len());

                sender.lock().unwrap().send(Vec::from(data)).unwrap();

                dispatcher_clone.lock().unwrap().send_async(|| {
                    //texture_clone.lock().unwrap().update_texture();
                });

                gst::FlowReturn::Ok
            })
            .build()
        );

        self.pipeline = Some(pipeline);
    }

    pub fn play(&mut self) {
        // Start playing
        if let Some(ref pipeline) = self.pipeline {
            let ret = pipeline.set_state(gst::State::Playing);
            assert_ne!(ret, gst::StateChangeReturn::Failure);
        }
    }

    pub fn on_loop_interation(&mut self) -> Result<()> {
        if let Some(ref receiver) = self.receiver {
            while let Ok(buffer) = receiver.try_recv() {
                let timespec = time::get_time();
                let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
                println!("buffer size: {}, thread id: {:?}, time: {:?}", buffer.len(), std::thread::current().id(), mills);
                self.texture.update_texture(buffer)?
            }
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        // Shutdown pipeline
        if let Some(ref pipeline) = self.pipeline {
            let ret = pipeline.set_state(gst::State::Null);
            assert_ne!(ret, gst::StateChangeReturn::Failure);
        }
    }
}

pub struct PlayerTexture {
    pub updated: Callback<i32>,
    texture_id: i32,
    width: u16,
    height: u16,
    drawing_context: Rc<RefCell<DrawingContext>>,
}

impl PlayerTexture {
    pub fn new(drawing_context: Rc<RefCell<DrawingContext>>) -> Self {
        PlayerTexture {
            updated: Callback::new(),
            texture_id: -1, width: 0, height: 0,
            drawing_context
        }
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    fn update_texture(&mut self, buffer: Vec<u8>) -> Result<()> {
        let timespec = time::get_time();
        let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
        println!("Dispatcher, thread id: {:?}, time: {:?}", std::thread::current().id(), mills);

        if self.texture_id == -1 {
            let drawing_context = &mut self.drawing_context.borrow_mut();
            self.texture_id = drawing_context.create_texture(&buffer, self.width, self.height, ColorFormat::RGBA, true)?;
        }
        else {
            let drawing_context = &mut self.drawing_context.borrow_mut();
            drawing_context.update_texture(self.texture_id, &buffer, 0, 0, self.width, self.height)?;
        }

        self.updated.emit(self.texture_id);

        Ok(())
    }
}
