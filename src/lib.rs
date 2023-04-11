#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use embedded_graphics_core::pixelcolor::RgbColor;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::Rgb888,
    Pixel,
};
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};

#[derive(Debug)]
pub enum UefiDisplayError {
    PixelRangeError(String),
    DisplayError(String),
}

pub struct UefiDisplay {
    /// 宽
    width: usize,
    // 长
    height: usize,
    // pixels
    pixels: Vec<BltPixel>,
}

impl UefiDisplay {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }

    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }

    pub fn flush(&self, gop: &mut GraphicsOutput) -> Result<(), UefiDisplayError> {
        match gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        }) {
            Ok(_) => Ok(()),
            Err(_) => Err(UefiDisplayError::DisplayError("draw pixel error".into())),
        }
    }
}

impl OriginDimensions for UefiDisplay {
    fn size(&self) -> Size {
        Size {
            width: self.width as u32,
            height: self.height as u32,
        }
    }
}

impl DrawTarget for UefiDisplay {
    type Color = Rgb888;
    type Error = UefiDisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            // 像素坐标
            let (x, y) = (point.x, point.y);
            // 坐标转换为pixels的索引
            if let Some(pixel) = self.pixel(x as usize, y as usize) {
                // 写入颜色
                pixel.red = color.r();
                pixel.green = color.g();
                pixel.blue = color.b();
            }
        }

        Ok(())
    }
}
