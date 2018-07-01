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
    pub counter: Property<i32>,
    pub counter2: Property<i32>
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel { counter: Property::new(10), counter2: Property::new(0) }
    }

    pub fn increase(&mut self) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(&mut self) {
        self.counter.change(|c| c - 1);
    }
}

impl View for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<MainViewModel>>) -> ViewData {
        // controls
        let btn1 = Button::control(Text::control("Decrease"));
        let btn2 = Button::control(Text::control("Increase"));
        let text1 = Text::control("");
        let text2 = Text::control("");

        // events
        btn1.borrow_mut().data.events.clicked.set_vm(view_model, |vm, _| { vm.decrease(); });
        btn2.borrow_mut().data.events.clicked.set_vm(view_model, |vm, _| { vm.increase(); });

        // bindings
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();
        let bindings = vec![
            text1.borrow_mut().data.properties.text.bind_c(&mut vm.counter, |counter| { format!("Counter {}", counter) } ),
            text2.borrow_mut().data.properties.text.bind_c(&mut vm.counter2, |counter| { format!("Counter2 {}", counter) } ),

            // test for two way binding            
            vm.counter2.bind(&mut vm.counter),
            vm.counter.bind(&mut vm.counter2),
        ];

        // layout
        let root_control = Horizontal::control(vec![
            text1, btn1, btn2, text2
        ]);

        ViewData {
            root_control: root_control,
            bindings: bindings,
        }
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let main_view_model = Rc::new(RefCell::new(MainViewModel::new()));
    app.set_root_view_model(&main_view_model);

    gst::init().unwrap();

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
        .build());
    let video_sink = video_app_sink.dynamic_cast::<gst::Element>().unwrap();

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new("test-pipeline");

    // Build the pipeline
    pipeline.add_many(&[&source, &video_sink]).unwrap();
    source.link(&video_sink).expect("Elements could not be linked.");

    // Modify the source's properties
    source.set_property_from_str("pattern", "smpte");

    // Start playing
    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    app.run();

    // Shutdown pipeline
    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
