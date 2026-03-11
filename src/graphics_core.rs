use crate::{ControllerInterface, Framebuffer, ResetInterface, SH8601ColorMode, Sh8601Driver};
use embedded_graphics::pixelcolor::{Gray8, Rgb565, Rgb666};
use embedded_graphics::prelude::*;
use embedded_graphics_core::pixelcolor::Rgb888;

macro_rules! impl_target {
    ($color_type:ident) => {
        impl<IFACE, RST, const WIDTH: usize, const HEIGHT: usize, const N: usize> DrawTarget
            for Sh8601Driver<IFACE, RST, $color_type, WIDTH, HEIGHT, N>
        where
            IFACE: ControllerInterface,
            RST: ResetInterface,
        {
            type Color = $color_type;
            // Drawing to the framebuffer in memory is infallible.
            // Errors happen during flush with SPI comms.
            type Error = core::convert::Infallible;

            /// Draws a single pixel to the internal framebuffer.
            fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
            where
                I: IntoIterator<Item = Pixel<Self::Color>>,
            {
                for Pixel(coord, color) in pixels.into_iter() {
                    match &mut self.framebuffer {
                        Framebuffer::Static(framebuffer) => framebuffer.set_pixel(coord, color),
                        Framebuffer::Heap(framebuffer) => framebuffer.set_pixel(coord, color),
                    }
                }
                Ok(())
            }
        }
    };
}

impl_target!(Rgb888);
impl_target!(Rgb666);
impl_target!(Rgb565);
impl_target!(Gray8);

// =========== embedded-graphics OriginDimensions Implementation ===========

impl<IFACE, RST, COLOR, const WIDTH: usize, const HEIGHT: usize, const N: usize> OriginDimensions
    for Sh8601Driver<IFACE, RST, COLOR, WIDTH, HEIGHT, N>
where
    IFACE: ControllerInterface,
    RST: ResetInterface,
    COLOR: SH8601ColorMode,
{
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}
