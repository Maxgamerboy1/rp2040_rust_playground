# pimoroni_unicorn
Simple rust wrapper for pimoroni unicorn - pio implementation for controlling the matrix

Makes use of `embedded_graphics` to allow ease of drawing to display.
Allows directly setting pixels too.

LED management is handled by the pico-unicorn example PIO program:
https://github.com/pimoroni/pimoroni-pico/blob/main/libraries/pico_unicorn/pico_unicorn.pio

Majority of work completed by 9names' [original implementation](https://github.com/9names/rp2040_rust_playground).