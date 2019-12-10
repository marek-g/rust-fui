pub type Result<T> = std::result::Result<T, failure::Error>;

mod video_player;
pub use video_player::*;

mod video_player_gl;
pub use video_player_gl::*;
