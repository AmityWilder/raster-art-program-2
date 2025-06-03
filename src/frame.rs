use std::marker::PhantomData;

use raylib::prelude::*;
use crate::draw_texture_custom;

pub struct Frame {
    buffer: RenderTexture2D,
    is_dirty: bool,
}

pub struct RaylibFrameMode<'a, 'b, T>(&'a mut T, PhantomData<&'b mut RenderTexture2D>);

impl<'a, 'b, T> std::ops::Deref for RaylibFrameMode<'a, 'b, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'b, T> std::ops::DerefMut for RaylibFrameMode<'a, 'b, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, 'b, T> Drop for RaylibFrameMode<'a, 'b, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::EndScissorMode();
            ffi::EndTextureMode();
        }
    }
}

impl<'a, 'b, T> RaylibDraw for RaylibFrameMode<'a, 'b, T> {}

impl Frame {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            buffer: rl.load_render_texture(&thread, rl.get_screen_width().try_into().unwrap(), rl.get_screen_height().try_into().unwrap()).unwrap(),
            is_dirty: true,
        }
    }

    pub fn resize(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        if width != self.buffer.texture.width || height != self.buffer.texture.height {
            self.buffer = rl.load_render_texture(&thread, width.try_into().unwrap(), height.try_into().unwrap()).unwrap();
            self.is_dirty = true;
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn begin_drawing<'a, 'b>(&'b mut self, rl: &'a mut RaylibHandle, _thread: &RaylibThread) -> RaylibFrameMode<'a, 'b, RaylibHandle> {
        self.is_dirty = true;
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        unsafe {
            ffi::BeginTextureMode(*self.buffer);
            ffi::BeginScissorMode(0, 0, width, height);
        }
        RaylibFrameMode(rl, PhantomData)
    }

    pub fn present<D: RaylibDraw>(&mut self, d: &mut D) {
        let rec = Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.buffer.texture.width as f32,
            height: self.buffer.texture.height as f32,
        };
        draw_texture_custom(d, &self.buffer, &rec, Color::WHITE);
        self.is_dirty = false;
    }
}
