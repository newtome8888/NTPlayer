pub mod components;
pub mod start_window;
pub mod video_window;
pub mod play_list_window;

use sdl2::ttf::Sdl2TtfContext;
use static_init::dynamic;

static DEFAULT_FONT_PATH: &'static str = "./assets/fonts/Roboto-Regular.ttf";

#[dynamic]
static TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();

pub trait NTWindow{
    /// Get the window id, this id is the id of sdl window
    fn id(&self)->u32;
}
