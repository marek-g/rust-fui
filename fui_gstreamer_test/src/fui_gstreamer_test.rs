#![windows_subsystem = "windows"]

extern crate time;
extern crate fui;
extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
use gst::prelude::*;

use fui::application::*;
use fui::controls::*;
use fui::layout::*;
use fui::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use Property;

struct MainViewModel {
    pub player: Player,
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel {
            player: Player::new(),
        }
    }

    pub fn play(&mut self) {
        self.player.open();
        self.player.play();
    }

    pub fn stop(&mut self) {
        self.player.stop();
    }
}

impl View for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<MainViewModel>>) -> ViewData {
        // controls
        let btn_play = Button::control(Text::control("Play"));
        let btn_stop = Button::control(Text::control("Stop"));
        let bitmap = Bitmap::control(-1);

        // events
        btn_play.borrow_mut().data.events.clicked.set_vm(view_model, |vm, _| { vm.play(); });
        btn_stop.borrow_mut().data.events.clicked.set_vm(view_model, |vm, _| { vm.stop(); });
        {
            let vm: &mut MainViewModel = &mut view_model.borrow_mut();
            vm.player.texture.lock().unwrap().updated.set_vm(&bitmap, |bitmap, texture_id| {
                bitmap.data.texture_id.set(texture_id);
                bitmap.set_is_dirty(true);
            });
        }

        // bindings
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();
        let bindings = vec![
        ];

        // layout
        let root_control = Horizontal::control(vec![
            btn_play, btn_stop, bitmap,
        ]);

        ViewData {
            root_control: root_control,
            bindings: bindings,
        }
    }
}

struct Player {
    pub texture: Arc<Mutex<PlayerTexture>>,
    pipeline: Option<gst::Pipeline>,
    dispatcher: Arc<Mutex<Dispatcher>>,
}

impl Player {
    pub fn new() -> Self {
        gst::init().unwrap();

        Player {
            texture: Arc::new(Mutex::new(PlayerTexture::new())),
            pipeline: None,
            dispatcher: Arc::new(Mutex::new(Dispatcher::for_current_thread())),
        }
    }

    pub fn open(&mut self) {
        println!("Main thread id: {:?}", std::thread::current().id());

        // Create the elements
        let source = gst::ElementFactory::make("videotestsrc", "source").expect("Could not create source element.");
        //let sink = gst::ElementFactory::make("glimagesink", "sink").expect("Could not create sink element");
        let video_sink = gst::ElementFactory::make("appsink", "sink").expect("Could not create sink element");
        let video_app_sink = video_sink.dynamic_cast::<gst_app::AppSink>().unwrap();
        video_app_sink.set_caps(&gst::Caps::new_simple(
            "video/x-raw",
            &[
                ("format", &"BGRA"),
                ("pixel-aspect-ratio", &gst::Fraction::from((1, 1))),
            ],
        ));

        let dispatcher_clone = self.dispatcher.clone();
        let texture_clone = self.texture.clone();
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

                dispatcher_clone.lock().unwrap().send_async(|| {
                    //texture_clone.lock().unwrap().update_texture();
                });

                gst::FlowReturn::Ok
            })
            .build()
        );

        let video_sink = video_app_sink.dynamic_cast::<gst::Element>().unwrap();

        // Create the empty pipeline
        let pipeline = gst::Pipeline::new("test-pipeline");

        // Build the pipeline
        pipeline.add_many(&[&source, &video_sink]).unwrap();
        source.link(&video_sink).expect("Elements could not be linked.");

        // Modify the source's properties
        source.set_property_from_str("pattern", "smpte");

        self.pipeline = Some(pipeline);
    }

    pub fn play(&mut self) {
        // Start playing
        if let Some(ref pipeline) = self.pipeline {
            let ret = pipeline.set_state(gst::State::Playing);
            assert_ne!(ret, gst::StateChangeReturn::Failure);
        }
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
}

impl PlayerTexture {
    pub fn new() -> Self {
        PlayerTexture {
            updated: Callback::new(),
            texture_id: -1,
        }
    }

    fn update_texture(&mut self) {
        let timespec = time::get_time();
        let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
        println!("Dispatcher, thread id: {:?}, time: {:?}", std::thread::current().id(), mills);
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let main_view_model = Rc::new(RefCell::new(MainViewModel::new()));
    app.set_root_view_model(&main_view_model);
 
    app.run();  
}
