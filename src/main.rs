#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use panic_probe as _;

#[cortex_m_rt::pre_init]
unsafe fn before_main() {
    // Soft-reset doesn't clear spinlocks. Clear the one used by critical-section
    // before we hit main to avoid deadlocks when using a debugger
    embassy_rp::pac::SIO.spinlock(31).write_value(1);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start");
    let p = embassy_rp::init(Default::default());

    let button = Input::new(p.PIN_3, Pull::Up);
    let mut led = Output::new(p.PIN_22, Level::Low);

    loop {
        if button.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
