extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
use self::gst::prelude::*;

pub fn create_pipeline_videotest() -> (gst::Pipeline, gst_app::AppSink) {
    let source = gst::ElementFactory::make("videotestsrc", "source").expect("Could not create source element.");
    source.set_property_from_str("pattern", "smpte");

    let video_sink = gst::ElementFactory::make("appsink", "sink").expect("Could not create sink element");
    let video_app_sink = video_sink.dynamic_cast::<gst_app::AppSink>().unwrap();
    video_app_sink.set_caps(&gst::Caps::new_simple(
        "video/x-raw",
        &[
            ("format", &"BGRA"),
            ("pixel-aspect-ratio", &gst::Fraction::from((1, 1))),
        ],
    ));

    let video_sink = video_app_sink.dynamic_cast::<gst::Element>().unwrap();

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new("test-pipeline");

    // Build the pipeline
    pipeline.add_many(&[&source, &video_sink]).unwrap();
    source.link(&video_sink).expect("Elements could not be linked.");

    let video_app_sink = video_sink.dynamic_cast::<gst_app::AppSink>().unwrap();

    (pipeline, video_app_sink)
}

pub fn create_pipeline_url(url: &str) -> (gst::Pipeline, gst_app::AppSink) {
    let source = gst::ElementFactory::make("uridecodebin", "source")
        .expect("Could not create uridecodebin element.");
    source.set_property_from_str("uri", url);

    let video_convert = gst::ElementFactory::make("videoconvert", "videoconvert")
        .expect("Could not create videoconvert element.");
    let audio_convert = gst::ElementFactory::make("audioconvert", "audioconvert")
        .expect("Could not create audioconvert element.");

    let video_sink = gst::ElementFactory::make("appsink", "videosink").expect("Could not create sink element");
    let audio_sink = gst::ElementFactory::make("autoaudiosink", "audiosink").expect("Could not create sink element.");

    let video_app_sink = video_sink.dynamic_cast::<gst_app::AppSink>().unwrap();
    video_app_sink.set_caps(&gst::Caps::new_simple(
        "video/x-raw",
        &[
            ("format", &"BGRA"),
            ("pixel-aspect-ratio", &gst::Fraction::from((1, 1))),
        ],
    ));

    let video_sink = video_app_sink.dynamic_cast::<gst::Element>().unwrap();

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new("test-pipeline");

    // Build the pipeline
    pipeline.add_many(&[&source, &video_convert, &video_sink, &audio_convert, &audio_sink]).unwrap();
    video_convert.link(&video_sink).expect("Elements could not be linked.");
    audio_convert.link(&audio_sink).expect("Elements could not be linked.");

    // Connect the pad-added signal
    let pipeline_clone = pipeline.clone();
    //let convert_clone = convert.clone();
    let video_sink_clone = video_convert.clone();
    let audio_sink_clone = audio_convert.clone();
    source.connect_pad_added(move |_, src_pad| {
        let pipeline = &pipeline_clone;
        let video_sink = &video_sink_clone;
        let audio_sink = &audio_sink_clone;

        println!(
            "Received new pad {} from {}",
            src_pad.get_name(),
            pipeline.get_name()
        );

        let new_pad_caps = src_pad
            .get_current_caps()
            .expect("Failed to get caps of new pad.");
        let new_pad_struct = new_pad_caps
            .get_structure(0)
            .expect("Failed to get first structure of caps.");
        let new_pad_type = new_pad_struct.get_name();

        println!("src pad type: {}", new_pad_type);

        let is_audio = new_pad_type.starts_with("audio/x-raw");
        let is_video = new_pad_type.starts_with("video/x-raw");

        if is_video {
            let sink_pad = video_sink.get_static_pad("sink")
                .expect("Failed to get static sink pad from convert");
            if sink_pad.is_linked() {
                println!("We are already linked. Ignoring.");
                return;
            }

            let ret = src_pad.link(&sink_pad);
            if ret != gst::PadLinkReturn::Ok {
                println!("Type is {} but link failed.", new_pad_type);
            } else {
                println!("Link succeeded (type {}).", new_pad_type);
            }
        }

        if is_audio {
            let sink_pad = audio_sink.get_static_pad("sink")
                .expect("Failed to get static sink pad from convert");
            if sink_pad.is_linked() {
                println!("We are already linked. Ignoring.");
                return;
            }

            let ret = src_pad.link(&sink_pad);
            if ret != gst::PadLinkReturn::Ok {
                println!("Type is {} but link failed.", new_pad_type);
            } else {
                println!("Link succeeded (type {}).", new_pad_type);
            }
        }
    });

    let video_app_sink = video_sink.dynamic_cast::<gst_app::AppSink>().unwrap();

    (pipeline, video_app_sink)
}
