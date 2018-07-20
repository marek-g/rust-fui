extern crate failure;
extern crate fui;
extern crate gstreamer_media;

extern crate gstreamer;
extern crate gstreamer_app;

pub type Result<T> = std::result::Result<T, failure::Error>;

mod video_player;
pub use video_player::*;

mod video_player_gl;
pub use video_player_gl::*;
