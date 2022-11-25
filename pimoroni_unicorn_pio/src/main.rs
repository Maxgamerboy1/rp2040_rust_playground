//! Simple test of pimoroni unicorn PIO library
//!
#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_hal::digital::v2::{InputPin, ToggleableOutputPin};
use panic_probe as _;
use pimoroni_unicorn_pio::UnicornPins;

use bsp::hal::{
    clocks::init_clocks_and_plls, pac, pio::PIOExt, sio::Sio, watchdog::Watchdog, Clock,
};
use rp_pico as bsp;

use embedded_graphics::{
    mono_font::{iso_8859_4::FONT_5X7, MonoTextStyleBuilder},
    pixelcolor::RgbColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let _clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = Delay::new(_core.SYST, _clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let unipins = UnicornPins {
        led_blank: pins.gpio11.into_mode(),
        led_latch: pins.gpio10.into_mode(),
        led_clock: pins.gpio9.into_mode(),
        led_data: pins.gpio8.into_mode(),
        row_0: pins.gpio22.into_mode(),
        row_1: pins.gpio21.into_mode(),
        row_2: pins.gpio20.into_mode(),
        row_3: pins.gpio19.into_mode(),
        row_4: pins.gpio18.into_mode(),
        row_5: pins.gpio17.into_mode(),
        row_6: pins.gpio16.into_mode(),
    };

    let mut uni = pimoroni_unicorn_pio::Unicorn::new(&mut pio, sm0, unipins);
    // Buttons
    let btn_a = pins.gpio12.into_pull_up_input();
    let mut _btn_b = pins.gpio13.into_floating_input();
    let mut _btn_x = pins.gpio14.into_floating_input();
    let mut _btn_y = pins.gpio15.into_floating_input();

    let mut led_pin = pins.led.into_push_pull_output();

    led_pin.toggle().unwrap();
    Rectangle::new(Point::new(0, 0), Size::new(7, 7))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::BLUE)
                .stroke_width(1)
                .fill_color(Rgb888::CSS_ORANGE_RED)
                .build(),
        )
        .draw(&mut uni)
        .unwrap();

    let clear = Rgb888::new(0, 0, 0);
    let text = Text::new(
        "MAX",
        Point::new(0, 5),
        MonoTextStyleBuilder::new()
            .background_color(Rgb888::BLUE)
            .font(&FONT_5X7)
            .text_color(Rgb888::RED)
            .build(),
    );

    loop {
        if btn_a.is_low().unwrap() {
            uni.clear(clear).unwrap();
            text.draw(&mut uni).unwrap();
        }

        uni.draw();
    }
}
