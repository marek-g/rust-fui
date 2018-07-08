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
