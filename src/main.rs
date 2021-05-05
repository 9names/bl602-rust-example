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
use panic_halt as _;

const I2C_CMDS: [[u8; 2]; 8] = [
    [0x0, 0x0],
    [0x1, 0x0],
    [0x04, 0x0],
    [0x05, 0x0],
    [0x06, 0x0],
    [0x07, 0x0],
    [0x14, 0xff],
    [0x15, 0xff],
];

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();

    let clocks = Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .uart_clk(UART_PLL_FREQ.Hz())
        .i2c_clk(10_000_000u32.Hz())
        .freeze(&mut parts.clk_cfg);

    let pin16 = parts.pin16.into_uart_sig0();
    let pin7 = parts.pin7.into_uart_sig7();
    let mux0 = parts.uart_mux0.into_uart0_tx();
    let mux7 = parts.uart_mux7.into_uart0_rx();

    let mut serial = Serial::uart0(
        dp.UART,
        Config::default().baudrate(2_000_000.Bd()),
        ((pin16, mux0), (pin7, mux7)),
        clocks,
    );
    let mut gpio5 = parts.pin5.into_floating_output();

    let sda = parts.pin1.into_i2c_sda();
    let scl = parts.pin2.into_i2c_scl();
    let mut i2c = hal::i2c::I2c::i2c(dp.I2C, (scl, sda), 200_000u32.Hz(), clocks);

    // Create a blocking delay function based on the current cpu frequency
    let mut d = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);
    serial.write_str("Starting i2c test\r\n").ok();

    for byte_pair in I2C_CMDS.iter() {
        let result = i2c.try_write(0x20, byte_pair);
        match result {
            Ok(_) => {
                serial.write_str("writing success\r\n").ok();
            }
            Err(e) => {
                serial.write_str(i2c_err_strings(e)).ok();
            }
        }
        d.try_delay_us(100).unwrap();
    }

    serial.write_str("Finished i2c test\r\n\r\n").ok();
    loop {
        gpio5.try_toggle().unwrap();
        d.try_delay_ms(1000).unwrap();
    }
}

fn i2c_err_strings(e: hal::i2c::Error) -> &'static str {
    match e {
        hal::i2c::Error::RxOverflow => "err RxOverflow\r\n",
        hal::i2c::Error::RxUnderflow => "err RxUnderflow\r\n",
        hal::i2c::Error::TxOverflow => "err TxOverflow\r\n",
        hal::i2c::Error::TxUnderflow => "err TxUnderflow\r\n",
        hal::i2c::Error::Timeout => "err Timeout\r\n",
    }
}
