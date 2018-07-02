#![windows_subsystem = "windows"]

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

use Property;

struct MainViewModel {
    pub texture_id: Property<i32>,
    player: Player,
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel {
            texture_id: Property::new(-1),
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

        // bindings
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();
        let bindings = vec![
            bitmap.borrow_mut().data.texture_id.bind(&mut vm.texture_id),
        ];

        // layout
        let root_control = Horizontal::control(vec![
            btn_play, btn_stop,
        ]);

        ViewData {
            root_control: root_control,
            bindings: bindings,
        }
    }
}

struct Player {
    pipeline: Option<gst::Pipeline>,
}

impl Player {
    pub fn new() -> Self {
        gst::init().unwrap();

        Player {
            pipeline: None,
        }
    }

    pub fn open(&mut self) {
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
        video_app_sink.set_callbacks(gst_app::AppSinkCallbacks::new()
            .new_sample(|app_sink| {
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

                print!("AppSink: New sample ({}x{}, size: {})\n", width, height, data.len());

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

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let main_view_model = Rc::new(RefCell::new(MainViewModel::new()));
    app.set_root_view_model(&main_view_model);
 
    app.run();  
}
