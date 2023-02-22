use std::{cell::UnsafeCell, mem::MaybeUninit};

use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

thread_local! {
    pub(in crate::ui::main_window) static CANVAS: UnsafeCell<MaybeUninit<Canvas<Window>>>  = UnsafeCell::new(MaybeUninit::zeroed());
    pub(in crate::ui::main_window) static TEXTURE_CREATOR: UnsafeCell<MaybeUninit<TextureCreator<WindowContext>>>  = UnsafeCell::new(MaybeUninit::zeroed());
}
