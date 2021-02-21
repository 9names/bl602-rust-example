#![no_std]
#![no_main]

use bl602_hal as hal;
use core::fmt::Write;
use hal::{
    clock::{Strict, SysclkFreq, UART_PLL_FREQ},
    pac,
    prelude::*,
    serial::*,
};
use panic_persist as _;
use bl602_exception as _;
use core::sync::atomic::{AtomicUsize, Ordering};
#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    // Set up all the clocks we need
    let clocks = Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .uart_clk(UART_PLL_FREQ.Hz())
        .freeze(&mut parts.clk_cfg);
    // Set up uart output. Since this microcontroller has a pin matrix,
    // we need to set up both the pins and the muxs
    let pin16 = parts.pin16.into_uart_sig0();
    let pin7 = parts.pin7.into_uart_sig7();
    let mux0 = parts.uart_mux0.into_uart0_tx();
    let mux7 = parts.uart_mux7.into_uart0_rx();

    // Configure our UART to 2MBaud, and use the pins we configured above
    let mut serial = Serial::uart0(
        dp.UART,
        Config::default().baudrate(2_000_000.Bd()),
        ((pin16, mux0), (pin7, mux7)),
        clocks,
    );
    // Also set up a pin as GPIO, to blink an LED
    let mut gpio5 = parts.pin5.into_pull_down_output();

    // If we rebooted due to panic, print the panic message
    if let Some(msg) = panic_persist::get_panic_message_bytes() {
        for c in msg.iter() {
            nb::block!(serial.try_write(*c)).ok();
        }
    }  

    // Print some characters to let you know we're running!
    nb::block!(serial.try_write(b'\r')).ok();
    nb::block!(serial.try_write(b'\n')).ok();
    nb::block!(serial.try_write(b'R')).ok();
    nb::block!(serial.try_write(b'U')).ok();
    nb::block!(serial.try_write(b'S')).ok();
    nb::block!(serial.try_write(b'T')).ok();
    nb::block!(serial.try_write(b'\r')).ok();
    nb::block!(serial.try_write(b'\n')).ok();
    nb::block!(serial.try_flush()).ok();

    // Create a blocking delay function based on the current cpu frequency
    let mut d = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);

    loop {
        // Okay, now lets have some fun.
        serial.write_str("LEDs on\r\n").unwrap();
        gpio5.try_set_high().unwrap();
        d.try_delay_ms(1000).unwrap();

        serial.write_str("LEDs off\r\n").unwrap();
        gpio5.try_set_low().unwrap();
        d.try_delay_ms(1000).unwrap();

        let a = AtomicUsize::new(0);
        let b = a.fetch_add(1, Ordering::Relaxed);
    }
}
