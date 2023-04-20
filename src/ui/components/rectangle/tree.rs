use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use sdl2::{pixels::Color, render::Canvas, ttf::Font, video::Window, rect::Rect};

use crate::{
    ui::{
        components::{MouseUpParam, TControl, DOUBLE_CLICK_INTERVAL},
        DEFAULT_FONT_PATH, TTF_CONTEXT,
    },
    util::error::SuperError,
};

use super::Rectangle;

static mut SELECTED_FONT_COLOR: Color = Color::BLUE;
static mut DEFAULT_FONT_COLOR: Color = Color::WHITE;
static mut FONT_SIZE: u16 = 10;

pub struct TreeView<V: Clone + PartialEq, E: Clone> {
    inner: Rectangle,
    root: Rc<RefCell<Node<V, E>>>,
    show_root: bool,
}

impl<V: Clone + PartialEq, E: Clone> TreeView<V, E> {
    pub fn new(
        value: V,
        text: &'static str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        Ok(Self {
            inner: Rectangle::new(x, y, width, height, canvas.clone())?,
            root: Rc::new(RefCell::new(Node::new(
                value,
                text,
                x,
                y,
                width,
                height,
                canvas.clone(),
            )?)),
            show_root: false,
        })
    }

    pub fn selected_font_color(&self) -> Color {
        unsafe { SELECTED_FONT_COLOR }
    }

    pub fn set_selected_font_color(&mut self, color: Color) {
        unsafe { SELECTED_FONT_COLOR = color };
    }

    pub fn default_font_color(&self) -> Color {
        unsafe { DEFAULT_FONT_COLOR }
    }

    pub fn set_default_font_color(&mut self, color: Color) {
        unsafe { DEFAULT_FONT_COLOR = color };
    }

    pub fn font_size(&self) -> u16 {
        unsafe { FONT_SIZE }
    }

    pub fn set_font_size(&mut self, size: u16) {
        unsafe { FONT_SIZE = size };
    }

    pub fn on_mouse_up(&mut self, params: &MouseUpParam) -> Result<bool, SuperError> {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        // Reset selected status of children
        self.root.borrow_mut().set_selected(false);

        Ok(true)
    }
}

impl<V: Clone + PartialEq, E: Clone> Deref for TreeView<V, E> {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<V: Clone + PartialEq, E: Clone> DerefMut for TreeView<V, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct Node<V: Clone + PartialEq, E: Clone> {
    inner: Rectangle,
    /// The value that can be the identifier of the node
    value: V,
    /// The text that will be displayed
    text: &'static str,
    /// The custom data
    extra: Option<E>,
    /// The parent node of current node
    parent: Option<Weak<RefCell<Node<V, E>>>>,
    /// The children of current node
    children: Vec<Node<V, E>>,
    /// The font of text
    font: Font<'static, 'static>,
    /// If current node is selected
    selected: bool,
}

impl<V: Clone + PartialEq, E: Clone> Node<V, E> {
    pub fn new(
        value: V,
        text: &'static str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        canvas: Rc<RefCell<Canvas<Window>>>,
    ) -> Result<Self, SuperError> {
        Ok(Self {
            inner: Rectangle::new(x, y, width, height, canvas)?,
            value: value,
            text: text,
            extra: None,
            parent: None,
            children: vec![],
            font: TTF_CONTEXT.load_font(DEFAULT_FONT_PATH, unsafe { FONT_SIZE })?,
            selected: false,
        })
    }

    /// Set the extra data of current node
    pub fn set_extra(&mut self, extra: E) {
        self.extra = Some(extra);
    }

    /// Get the reference of extra data
    pub fn extra(&self) -> Option<&E> {
        self.extra.as_ref()
    }

    /// Get the mutable reference of extra data
    pub fn extra_mut(&mut self) -> Option<&mut E> {
        self.extra.as_mut()
    }

    /// Set the display text of current node
    pub fn set_text(&mut self, text: &'static str) {
        self.text = text;
    }

    /// Get the text of current node
    pub fn text(&self) -> &'static str {
        self.text
    }

    /// Get the value of current node
    pub fn value(&self) -> V {
        self.value.clone()
    }

    /// Get the parent of current node
    pub fn parent(&self) -> Option<Rc<RefCell<Node<V, E>>>> {
        self.parent.as_ref().and_then(|p| p.upgrade())
    }

    /// Set the parent of current node
    pub fn set_parent(&mut self, parent: Option<Weak<RefCell<Node<V, E>>>>) {
        self.parent = parent;
    }

    /// Get the selected status of current node
    pub fn selected(&self) -> bool {
        self.selected
    }

    /// Set the selected state of current node and its children
    pub fn set_selected(&mut self, selected: bool) {
        match selected {
            true => {
                // If current node is selected, all its ancestors should be selected
                self.selected = true;
                if let Some(parent) = self.parent() {
                    parent.borrow_mut().set_selected(true);
                }
            }
            false => {
                if self.children.len() == 0 {}

                // If current node is deselected, all its descendants should be deselected
                for child in &mut self.children {
                    child.set_selected(false);
                }
            }
        }
    }

    pub fn render<F>(&mut self) -> Result<(), SuperError> {
        let mut canvas = self.canvas.borrow_mut();
        let (x, y) = self.inner.content_position();
        let color = if self.selected || self.is_cursorin() {
            unsafe { SELECTED_FONT_COLOR }
        } else {
            unsafe { DEFAULT_FONT_COLOR }
        };
        let sfs = self.font.render(self.text).blended(color)?;
        let tc = canvas.texture_creator();
        let texture = tc.create_texture_from_surface(sfs)?;
        let query = texture.query();
        let width = query.width;
        let height = query.height;
        
        let rect = Rect::new(x, y, width, height);
        canvas.copy(&texture, None, rect)?;

        Ok(())
    }

    pub fn on_mouse_up<F>(&mut self, params: &MouseUpParam, f: F) -> Result<bool, SuperError>
    where
        F: FnOnce(),
    {
        if !self.inner.on_mouse_up(params)? {
            return Ok(false);
        }

        self.set_selected(true);

        if self.preclick.elapsed().as_millis() < DOUBLE_CLICK_INTERVAL {
            // Double click, play current item
        }

        Ok(true)
    }
}

impl<V: Clone + PartialEq, E: Clone> Deref for Node<V, E> {
    type Target = Rectangle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<V: Clone + PartialEq, E: Clone> DerefMut for Node<V, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
