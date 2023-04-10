#![no_std]

extern crate alloc;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{IntoStorage, Rgb888},
    Pixel,
};
use uefi::proto::console::gop::FrameBuffer;

#[derive(Debug)]
pub struct Unsupported(());
impl Unsupported {
    pub fn new<T>(_: T) -> Self {
        Self(())
    }
}

pub struct UefiDisplay<'a> {
    /// UEFI FrameBuffer
    frame_buffer: &'a mut FrameBuffer<'a>,
    stride: u32,
    size: (u32, u32),
}

///  frame_buffer must be uefi framebuffer
impl<'a> UefiDisplay<'a> {
    pub fn new(frame_buffer: &'a mut FrameBuffer<'a>, stride: u32, size: (u32, u32)) -> Self {
        Self {
            frame_buffer,
            stride,
            size,
        }
    }
}

impl<'a> OriginDimensions for UefiDisplay<'a> {
    fn size(&self) -> Size {
        let (width, height) = self.size;
        Size { width, height }
    }
}

impl<'a> DrawTarget for UefiDisplay<'a> {
    type Color = Rgb888;
    type Error = Unsupported;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            let bytes = color.into_storage();
            let stride = self.stride as u64;
            let (x, y) = (point.x as u64, point.y as u64);
            let index: usize = (((y * stride) + x) * 4)
                .try_into()
                .map_err(Unsupported::new)?;

            // copy from FrameBuffer
            unsafe { self.frame_buffer.write_value(index, bytes) };
        }

        Ok(())
    }
}
