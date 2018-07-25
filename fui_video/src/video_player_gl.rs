extern crate failure;
extern crate time;
extern crate gstreamer as gst;
extern crate gstreamer_sys as gst_ffi;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_video as gst_video;
extern crate gstreamer_video_sys as gst_video_ffi;
extern crate fui;
extern crate std;
extern crate drawing;
extern crate drawing_gl;
extern crate glutin;
extern crate winit;
extern crate glib;
extern crate gl;

pub type Result<T> = std::result::Result<T, failure::Error>;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{ Arc, Mutex };
use std::sync::mpsc::*;
use self::gst::prelude::*;
use fui::*;
use gstreamer_media;
use self::drawing::backend::WindowTargetExt;
use self::glutin::os::GlContextExt;
use self::glutin::os::unix::RawHandle::Glx;
use self::glutin::api::glx::Context;
use self::winit::os::unix::EventsLoopExt;
use self::winit::os::unix::x11::XConnection;
use self::glib::Value;
use self::gl::types::*;

pub struct PlayerGl {
    pub texture: PlayerTexture,
    xconnection: Arc<XConnection>,
    drawing_context: Rc<RefCell<DrawingContext>>,
    window_manager: Rc<RefCell<WindowManager>>,
    pipeline: Option<self::gst::Pipeline>,
    dispatcher: Arc<Mutex<Dispatcher>>,
    receiver: Option<Receiver<GLuint>>,
    events_loop_proxy: winit::EventsLoopProxy,
}

impl PlayerGl {
    pub fn new(drawing_context: &Rc<RefCell<DrawingContext>>,
        window_manager: &Rc<RefCell<WindowManager>>,
        events_loop: &winit::EventsLoop) -> Result<Self> {
        gst::init()?;

        let xconnection = match events_loop.get_xlib_xconnection() {
            Some(xconnection) => xconnection,
            None => return Err(::failure::err_msg("Cannot find X11 Connection (Display)!")),
        };

        Ok(PlayerGl {
            texture: PlayerTexture::new(drawing_context.clone()),
            xconnection: xconnection,
            drawing_context: drawing_context.clone(),
            window_manager: window_manager.clone(),
            pipeline: None,
            dispatcher: Arc::new(Mutex::new(Dispatcher::for_current_thread())),
            receiver: None,
            events_loop_proxy: events_loop.create_proxy(),
        })
    }

    pub fn open(&mut self) -> Result<()> {
        println!("Main thread id: {:?}", std::thread::current().id());

        let (sender, receiver) = channel();
        self.receiver = Some(receiver);
        let sender = Arc::new(Mutex::new(sender));

        let main_window_id = self.window_manager.borrow().get_main_window_id();
        if let Some(main_window_id) = main_window_id {
            if let Some(main_window) = self.window_manager.borrow_mut().get_windows_mut().get(&main_window_id) {
                let window_drawing_target = main_window.get_drawing_target();
                let context = window_drawing_target.get_context();

                // Create the elements
                //let (pipeline, video_app_sink) = pipeline_factory::create_pipeline_videotest();
                //self.texture.set_size(320, 240);
                let (pipeline, video_sink) = gstreamer_media::create_opengl_pipeline_url(
                    "http://ftp.nluug.nl/pub/graphics/blender/demo/movies/Sintel.2010.720p.mkv",
                    &context, &self.xconnection);
                self.texture.set_size(1280, 544);

                let dispatcher_clone = self.dispatcher.clone();
                let events_loop_proxy_clone = self.events_loop_proxy.clone();

                video_sink.connect("client-reshape", false, move |args| {
                    println!("client-reshape! {:?}", args);
                    Some(Value::from(&true))
                }).unwrap();

                video_sink.connect("client-draw", false, move |args| {
                    println!("client-draw! {:?}", args);
                    let sample = args[2].get::<gst::Sample>().expect("Invalid argument - GstSample expected.");
                    if let (Some(buffer), Some(caps)) = (sample.get_buffer(), sample.get_caps()) {
                        println!("caps: {}", caps.to_string());
                        if let Some(video_info) = gst_video::VideoInfo::from_caps(&caps) {
                            println!("video_info: {:?}", video_info);

                            let texture_id = unsafe {
                                use video_player_gl::glib::translate::ToGlibPtr;
                                use std::mem;
                                use self::glib::translate::from_glib;
                                const GST_MAP_GL: u32 = 131072u32;

                                let mut frame: gst_video_ffi::GstVideoFrame = mem::zeroed();
                                let res: bool = from_glib(gst_video_ffi::gst_video_frame_map(
                                    &mut frame,
                                    video_info.to_glib_none().0 as *mut _,
                                    buffer.to_glib_none().0,
                                    mem::transmute(
                                        gst_video_ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits() |
                                            GST_MAP_GL,
                                    ),
                                ));

                                if !res {
                                    Err(buffer)
                                } else {
                                    let texture_id = *(frame.data[0] as *const GLuint);
                                    gst_video_ffi::gst_video_frame_unmap(&mut frame);
                                    Ok(texture_id)
                                }
                            };

                            if let Ok(texture_id) = texture_id {
                                println!("texture_id: {}", texture_id);
                               
                                sender.lock().unwrap().send(texture_id).unwrap();

                                events_loop_proxy_clone.wakeup().unwrap();;
                                //dispatcher_clone.lock().unwrap().send_async(|| {});
                            }
                        }
                        /*let map = buffer.into_mapped_buffer_readable().unwrap();
                        let data = map.as_slice();
                        //println!("data {:?}", data);
                        let texture_id = unsafe { *(data.as_ptr() as *const i64) };
                        println!("texture_id {}", texture_id);*/
                    }
                    Some(Value::from(&true))
                }).unwrap();

                /*video_app_sink.set_callbacks(gst_app::AppSinkCallbacks::new()
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
                );*/

                self.pipeline = Some(pipeline);

                Ok(())
            } else {
                Err(::failure::err_msg("Cannot find GL Context for the main window!"))
            }
        } else {
            Err(::failure::err_msg("Cannot find GL Context. There is no window!"))
        }
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
            while let Ok(texture_id) = receiver.try_recv() {
                let timespec = time::get_time();
                let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
                //println!("buffer size: {}, thread id: {:?}, time: {:?}", buffer.len(), std::thread::current().id(), mills);
                self.texture.update_texture(texture_id)?
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

    fn update_texture(&mut self, texture_id: GLuint) -> Result<()> {
        let timespec = time::get_time();
        let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
        println!("Dispatcher, thread id: {:?}, time: {:?}", std::thread::current().id(), mills);

        {
            let texture = drawing_gl::GlTexture::from_external(texture_id, self.width, self.height, ColorFormat::RGBA);
            let drawing_context = &mut self.drawing_context.borrow_mut();
            let resources = drawing_context.get_resources_mut();
        
            if self.texture_id == -1 {
                self.texture_id = resources.get_next_texture_id();
            }

            resources.textures_mut().insert(self.texture_id, texture);
        }
    
        self.updated.emit(self.texture_id);

        Ok(())
    }
}
