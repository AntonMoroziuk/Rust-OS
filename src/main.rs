#![no_std]
#![no_main]

// pick a panicking behavior
use crate::gpio::GPIO_PIN_5;
use crate::gpio::GPIO_MODE_OUTPUT;
use crate::gpio::GPIO_NO_PUPD;
use crate::gpio::GPIOA;


use crate::gpio::GPIO_SPEED_LOW;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

mod gpio;
mod rcc;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    rcc::rcc_init_clocks();
    rcc::rcc_gpio_set(rcc::IoPort::A, true);
    rcc::rcc_gpio_set(rcc::IoPort::B, true);
    rcc::rcc_gpio_set(rcc::IoPort::C, true);
    rcc::rcc_uart_set(rcc::UartPort::UartPort2, true);
    rcc::rcc_tim_set(rcc::RccTim::RccTim1, true);

    let pa5_config = gpio::GPIO_config_s {
        pin: GPIO_PIN_5,
        mode: GPIO_MODE_OUTPUT,
        pull: GPIO_NO_PUPD,
        speed: GPIO_SPEED_LOW,
        alternate: 0,
    };
    gpio::gpio_init(gpio::GPIOA, &pa5_config);

    // Enable LED
    gpio::gpio_set(gpio::GPIO_pin_s {port: GPIOA, pin: GPIO_PIN_5}, 1);
    loop {

    }
}
