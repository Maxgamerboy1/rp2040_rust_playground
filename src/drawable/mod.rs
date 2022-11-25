#[cfg(feature = "graphics")]
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::Size,
    geometry::{Dimensions, OriginDimensions},
    pixelcolor::Rgb888,
    prelude::*,
};
use rp_pico::hal::{pio::{ValidStateMachine, PIOExt}, gpio::FunctionConfig};

use crate::{Unicorn, WIDTH, HEIGHT};

#[cfg(feature = "graphics")]
impl<'pio, P, SM> DrawTarget for Unicorn<'pio, P, SM>
where
    P: PIOExt + FunctionConfig,
    SM: ValidStateMachine<PIO = P>,
{
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();
        pixels
            .into_iter()
            .filter(|Pixel(pos, _color)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| self.set_pixel(pos, color));
        Ok(())
    }
}

impl<'pio, P, SM> OriginDimensions for Unicorn<'pio, P, SM>
where
    P: PIOExt + FunctionConfig,
    SM: ValidStateMachine<PIO = P>,
{
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}