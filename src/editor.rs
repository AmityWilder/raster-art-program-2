use raylib::prelude::*;
use crate::{brush::Brush, frame::Frame};

pub mod art;
pub mod color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorID {
    Art,
    Color,
}

pub trait Editor {
    /// Indicate that a full redraw is necessary
    fn mark_dirty(&mut self);

    /// Test if a full redraw is queued
    fn is_dirty(&self) -> bool;

    /// The editor is performing an action that still belongs to it even if the mouse exits its container.
    fn is_focused(&self) -> bool;

    /// When awake, tick the editor as if it is either focused or can be
    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, brush: &mut Brush, viewport: Rectangle, frame: &mut Frame, is_awake: bool);
}
