#![no_std]
#![no_main]

mod ssd1306;

use ssd1306::Ssd1306;

use panic_halt as _;

use riscv_rt::entry;

use esp32c3_hal::{
    clock::ClockControl,
    pac::{Peripherals},
    prelude::*,
    system::SystemExt,
    timer::TimerGroup,
    Rtc, IO, i2c,
};


#[entry]
fn main() -> ! {
    const CLOCK_FREQ: fugit::HertzU32 = fugit::HertzU32::kHz(100);
    const SSD1306_ADDR: u8 = 0x3C;

    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT and the RTC WDT.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let mut timer0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer1 = TimerGroup::new(peripherals.TIMG1, &clocks);

    timer0.wdt.disable();
    timer1.wdt.disable();
    rtc.swd.disable();
    rtc.rwdt.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let scl = io.pins.gpio6.into_open_drain_output();
    let sda = io.pins.gpio7.into_open_drain_output();

    let mut _i2c = i2c::I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        CLOCK_FREQ,
        &mut system.peripheral_clock_control,
        &clocks
    ).unwrap();

    let mut ssd1306 = Ssd1306::new(_i2c, SSD1306_ADDR);
    ssd1306.setup().unwrap();
    ssd1306.draw_checkerboard().unwrap();
    loop{

    }
}