// mod list_config;

// use std::{
//     cell::RefCell,
//     ops::{Deref, DerefMut},
//     rc::Rc,
// };

// use sdl2::{
//     image::LoadSurface,
//     pixels::Color,
//     render::{BlendMode, Canvas},
//     surface::Surface,
//     video::{Window, WindowPos},
//     VideoSubsystem,
// };

// use self::list_config::PlayList;
// use crate::{util::error::SuperError, APP_NAME, LOGO_PATH};
// use super::{
//     components::{
//         rectangle::Rectangle, MouseDownParam, MouseMotionParam, MouseUpParam, MouseWheelParam,
//         TControl,
//     },
//     NTWindow,
// };

// pub struct PlayListWindow {
//     pub id: u32,
//     inner: Rectangle,
//     data: PlayList,
// }

// impl PlayListWindow {
//     pub fn new(sys: &VideoSubsystem) -> Result<Self, SuperError> {
//         let wind = Self::prepare_window(sys)?;
//         let window_id = wind.id();
//         let (x, y) = wind.position();
//         let (width, height) = wind.size();

//         let canvas = Self::prepare_canvas(wind)?;
//         let rc_canvas = Rc::new(RefCell::new(canvas));

//         let mut inst = Self {
//             id: window_id,
//             inner: Rectangle::new(x, y, width, height, rc_canvas)?,
//         };

//         inst.render()?;

//         Ok(inst)
//     }

//     pub fn show(&mut self) -> Result<(), SuperError> {
//         self.canvas_mut().window_mut().show();
//         self.render()?;

//         Ok(())
//     }

//     pub fn hide(&mut self) {
//         self.canvas_mut().window_mut().hide();
//     }

//     fn prepare_window(sys: &VideoSubsystem) -> Result<Window, SuperError> {
//         let mut wind = sys
//             .window("NT Player", 200, 200)
//             .borderless()
//             .allow_highdpi()
//             .position_centered()
//             .resizable()
//             .opengl()
//             .build()?;

//         wind.set_title(APP_NAME)?;

//         let logo = Surface::from_file(LOGO_PATH)?;
//         wind.set_icon(logo);

//         Ok(wind)
//     }

//     fn prepare_canvas(wind: Window) -> Result<Canvas<Window>, SuperError> {
//         let mut canvas = wind.into_canvas().build()?;
//         canvas.set_draw_color(Color::RGB(200, 200, 200));
//         canvas.set_blend_mode(BlendMode::Blend);
//         canvas.clear();
//         canvas.present();

//         Ok(canvas)
//     }

//     pub fn on_mouse_down(&mut self, params: &MouseDownParam) -> Result<bool, SuperError> {
//         if params.window_id != self.id {
//             return Ok(false);
//         }
//         Ok(true)
//     }

//     pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
//         if params.window_id != self.id {
//             return Ok(false);
//         }

//         Ok(true)
//     }

//     pub fn on_mouse_motion(&mut self, params: &MouseMotionParam) -> Result<bool, SuperError> {
//         if params.window_id != self.id {
//             return Ok(true);
//         }

//         Ok(true)
//     }

//     pub fn on_mouse_wheel(&mut self, params: &MouseWheelParam) -> Result<bool, SuperError> {
//         if params.window_id != self.id {
//             return Ok(false);
//         }
//         Ok(true)
//     }

//     pub fn set_size(&mut self, width: u32, height: u32) {}

//     pub fn set_position(&mut self, x: WindowPos, y: WindowPos) {}

//     pub fn render(&mut self) -> Result<bool, SuperError> {
//         // Display on screen
//         self.canvas_mut().present();

//         Ok(true)
//     }
// }

// impl NTWindow for PlayListWindow {
//     fn id(&self) -> u32 {
//         self.id
//     }
// }

// impl Deref for PlayListWindow {
//     type Target = Rectangle;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl DerefMut for PlayListWindow {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }
