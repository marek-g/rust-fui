use anyhow::{format_err, Result};
use fui_app::*;
use fui_core::*;
use gl::types::*;
use glib::Value;
use gstreamer::prelude::*;
use media_gstreamer;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};

//#[cfg(target_os = "linux")]
//use self::glutin::os::unix::RawHandle::Glx;
//#[cfg(target_os = "linux")]
//use self::winit::platform::unix::x11::XConnection;
//#[cfg(target_os = "linux")]
//use self::winit::platform::unix::EventsLoopExt;

#[cfg(target_os = "linux")]
use winit::platform::unix::WindowExtUnix;

pub struct PlayerGl {
    pub texture: PlayerTexture,
    drawing_context: Rc<RefCell<fui_app::DrawingContext>>,
    window_manager: Rc<RefCell<WindowManager>>,
    pipeline: Option<gstreamer::Pipeline>,
    dispatcher: Arc<Mutex<Dispatcher>>,
    receiver: Option<Receiver<GLuint>>,
    event_loop_proxy: winit::event_loop::EventLoopProxy<()>,
}

impl PlayerGl {
    #[cfg(target_os = "linux")]
    pub fn new(
        drawing_context: &Rc<RefCell<fui_app::DrawingContext>>,
        window_manager: &Rc<RefCell<WindowManager>>,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<Self> {
        gstreamer::init()?;

        Ok(PlayerGl {
            texture: PlayerTexture::new(drawing_context.clone()),
            drawing_context: drawing_context.clone(),
            window_manager: window_manager.clone(),
            pipeline: None,
            dispatcher: Arc::new(Mutex::new(Dispatcher::for_current_thread())),
            receiver: None,
            event_loop_proxy: event_loop.create_proxy(),
        })
    }

    #[cfg(target_os = "windows")]
    pub fn new(
        drawing_context: &Rc<RefCell<fui_app::DrawingContext>>,
        window_manager: &Rc<RefCell<WindowManager>>,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<Self> {
        gstreamer::init()?;

        Ok(PlayerGl {
            texture: PlayerTexture::new(drawing_context.clone()),
            drawing_context: drawing_context.clone(),
            window_manager: window_manager.clone(),
            pipeline: None,
            dispatcher: Arc::new(Mutex::new(Dispatcher::for_current_thread())),
            receiver: None,
            event_loop_proxy: event_loop.create_proxy(),
        })
    }

    #[cfg(target_os = "linux")]
    pub fn open(&mut self) -> Result<()> {
        println!("Main thread id: {:?}", std::thread::current().id());

        let (sender, receiver) = channel();
        self.receiver = Some(receiver);
        let sender = Arc::new(Mutex::new(sender));

        let main_window_id = self.window_manager.borrow().get_main_window_id();
        if let Some(main_window_id) = main_window_id {
            if let Some(main_window) = self
                .window_manager
                .borrow_mut()
                .get_windows_mut()
                .get(&main_window_id)
            {
                let window = main_window.window.borrow();
                let window_drawing_target = window.get_drawing_target();
                let context = window_drawing_target.get_context();

                let window = window_drawing_target.get_window();
                let xconnection = window.xlib_xconnection().unwrap();

                // Create the elements
                //let (pipeline, video_app_sink) = pipeline_factory::create_pipeline_videotest();
                //self.texture.set_size(320, 240);
                let (pipeline, video_sink) = media_gstreamer::create_opengl_pipeline_url(
                    "http://ftp.nluug.nl/pub/graphics/blender/demo/movies/Sintel.2010.720p.mkv",
                    &context,
                    &xconnection,
                );
                self.texture.set_size(1280, 544);

                video_sink
                    .connect("client-reshape", false, move |args| {
                        println!("client-reshape! {:?}", args);
                        Some(Value::from(&true))
                    })
                    .unwrap();

                let dispatcher_clone = self.dispatcher.clone();
                let event_loop_proxy_clone = self.event_loop_proxy.clone();

                video_sink
                    .connect("client-draw", false, move |args| {
                        println!("client-draw! {:?}", args);
                        let sample = args[2]
                            .get::<gstreamer::Sample>()
                            .unwrap()
                            .expect("Invalid argument - GstSample expected.");
                        if let (Some(buffer), Some(caps)) = (sample.get_buffer(), sample.get_caps())
                        {
                            println!("caps: {}", caps.to_string());
                            if let Ok(video_info) = gstreamer_video::VideoInfo::from_caps(&caps) {
                                println!("video_info: {:?}", video_info);

                                let texture_id = unsafe {
                                    use glib::translate::from_glib;
                                    use glib::translate::ToGlibPtr;
                                    use std::mem;
                                    const GST_MAP_GL: u32 = 131072u32;

                                    // TODO: can we use from_buffer_readable_gl() instead?
                                    // https://gitlab.freedesktop.org/sjakthol/gstreamer-rs/commit/43f5a10f9c75c69fadccdf9d88e0102bd0ecaa5b
                                    let mut frame: gstreamer_video_sys::GstVideoFrame =
                                        mem::zeroed();
                                    let res: bool =
                                        from_glib(gstreamer_video_sys::gst_video_frame_map(
                                            &mut frame,
                                            video_info.to_glib_none().0 as *mut _,
                                            buffer.as_mut_ptr(),
                                            mem::transmute(
                                                gstreamer_video_sys::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                                                    | gstreamer_sys::GST_MAP_READ
                                                    | GST_MAP_GL,
                                            ),
                                        ));

                                    if !res {
                                        Err(buffer)
                                    } else {
                                        let texture_id = *(frame.data[0] as *const GLuint);
                                        gstreamer_video_sys::gst_video_frame_unmap(&mut frame);
                                        Ok(texture_id)
                                    }
                                };

                                if let Ok(texture_id) = texture_id {
                                    println!("texture_id: {}", texture_id);

                                    sender.lock().unwrap().send(texture_id).unwrap();

                                    // TODO: why send_event() doesn't compile?!!
                                    //event_loop_proxy_clone.send_event(()).unwrap();
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
                    })
                    .unwrap();

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
                Err(format_err!("Cannot find GL Context for the main window!",))
            }
        } else {
            Err(format_err!("Cannot find GL Context. There is no window!",))
        }
    }

    #[cfg(target_os = "windows")]
    pub fn open(&mut self) -> Result<()> {
        println!("Main thread id: {:?}", std::thread::current().id());

        let (sender, receiver) = channel();
        self.receiver = Some(receiver);
        let sender = Arc::new(Mutex::new(sender));

        let main_window_id = self.window_manager.borrow().get_main_window_id();
        if let Some(main_window_id) = main_window_id {
            if let Some(main_window) = self
                .window_manager
                .borrow_mut()
                .get_windows_mut()
                .get(&main_window_id)
            {
                let window_drawing_target = main_window.get_drawing_target();
                let context = window_drawing_target.get_context();

                // Create the elements
                //let (pipeline, video_app_sink) = pipeline_factory::create_pipeline_videotest();
                //self.texture.set_size(320, 240);
                let (pipeline, video_sink) = gstreamer_media::create_opengl_pipeline_url(
                    "http://ftp.nluug.nl/pub/graphics/blender/demo/movies/Sintel.2010.720p.mkv",
                    &context,
                );
                self.texture.set_size(1280, 544);

                let dispatcher_clone = self.dispatcher.clone();
                let event_loop_proxy_clone = self.event_loop_proxy.clone();

                video_sink
                    .connect("client-reshape", false, move |args| {
                        println!("client-reshape! {:?}", args);
                        //Some(Value::from(&true))
                        Some(Value::from(&false))
                    })
                    .unwrap();

                video_sink
                    .connect("client-draw", false, move |args| {
                        println!("client-draw! {:?}", args);
                        let sample = args[2]
                            .get::<gstreamer::Sample>()
                            .unwrap()
                            .expect("Invalid argument - GstSample expected.");
                        if let (Some(buffer), Some(caps)) = (sample.get_buffer(), sample.get_caps())
                        {
                            println!("caps: {}", caps.to_string());
                            if let Ok(video_info) = gstreamer_video::VideoInfo::from_caps(&caps) {
                                println!("video_info: {:?}", video_info);

                                let texture_id = unsafe {
                                    use glib::translate::from_glib;
                                    use glib::translate::ToGlibPtr;
                                    use std::mem;
                                    const GST_MAP_GL: u32 = 131072u32;

                                    // TODO: can we use from_buffer_readable_gl() instead?
                                    // https://gitlab.freedesktop.org/sjakthol/gstreamer-rs/commit/43f5a10f9c75c69fadccdf9d88e0102bd0ecaa5b
                                    let mut frame: gstreamer_video_sys::GstVideoFrame =
                                        mem::zeroed();
                                    let res: bool =
                                        from_glib(gstreamer_video_sys::gst_video_frame_map(
                                            &mut frame,
                                            video_info.to_glib_none().0 as *mut _,
                                            buffer.as_mut_ptr(),
                                            mem::transmute(
                                                gstreamer_video_sys::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                                                    | gstreamer_sys::GST_MAP_READ
                                                    | GST_MAP_GL,
                                            ),
                                        ));

                                    if !res {
                                        Err(buffer)
                                    } else {
                                        let texture_id = *(frame.data[0] as *const GLuint);
                                        gstreamer_video_sys::gst_video_frame_unmap(&mut frame);
                                        Ok(texture_id)
                                    }
                                };

                                if let Ok(texture_id) = texture_id {
                                    println!("texture_id: {}", texture_id);

                                    sender.lock().unwrap().send(texture_id).unwrap();

                                    // TODO: why send_event() doesn't compile?!!
                                    //event_loop_proxy_clone.send_event(()).unwrap();
                                    //dispatcher_clone.lock().unwrap().send_async(|| {});
                                }
                            }
                            /*let map = buffer.into_mapped_buffer_readable().unwrap();
                            let data = map.as_slice();
                            //println!("data {:?}", data);
                            let texture_id = unsafe { *(data.as_ptr() as *const i64) };
                            println!("texture_id {}", texture_id);*/
                        }
                        //Some(Value::from(&true))
                        Some(Value::from(&false))
                    })
                    .unwrap();

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
                Err(format_err!("Cannot find GL Context for the main window!",))
            }
        } else {
            Err(format_err!("Cannot find GL Context. There is no window!",))
        }
    }

    pub fn play(&mut self) {
        // Start playing
        if let Some(ref pipeline) = self.pipeline {
            let ret = pipeline.set_state(gstreamer::State::Playing);
            assert_ne!(ret, Err(gstreamer::StateChangeError));
        }
    }

    pub fn on_loop_interation(&mut self) -> Result<()> {
        if let Some(ref receiver) = self.receiver {
            while let Ok(texture_id) = receiver.try_recv() {
                let timespec = time::Time::now();
                let mills: f64 = timespec.second() as f64
                    + (timespec.nanosecond() as f64 / 1000.0 / 1000.0 / 1000.0);
                //println!("buffer size: {}, thread id: {:?}, time: {:?}", buffer.len(), std::thread::current().id(), mills);
                self.texture.update_texture(texture_id)?
            }
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        // Shutdown pipeline
        if let Some(ref pipeline) = self.pipeline {
            let ret = pipeline.set_state(gstreamer::State::Null);
            assert_ne!(ret, Err(gstreamer::StateChangeError));
        }
    }
}

pub struct PlayerTexture {
    pub updated: Callback<i32>,
    texture_id: i32,
    width: u16,
    height: u16,
    drawing_context: Rc<RefCell<fui_app::DrawingContext>>,
}

impl PlayerTexture {
    pub fn new(drawing_context: Rc<RefCell<fui_app::DrawingContext>>) -> Self {
        PlayerTexture {
            updated: Callback::empty(),
            texture_id: -1,
            width: 0,
            height: 0,
            drawing_context,
        }
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    fn update_texture(&mut self, texture_id: GLuint) -> Result<()> {
        let timespec = time::Time::now();
        let mills: f64 =
            timespec.second() as f64 + (timespec.nanosecond() as f64 / 1000.0 / 1000.0 / 1000.0);
        println!(
            "Dispatcher, thread id: {:?}, time: {:?}",
            std::thread::current().id(),
            mills
        );

        {
            let texture = drawing_gl::GlTexture::from_external(
                texture_id,
                self.width,
                self.height,
                ColorFormat::RGBA,
            );
            let mut drawing_context = self.drawing_context.borrow_mut();
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
