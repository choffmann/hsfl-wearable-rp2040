#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use embedded_hal_0_2::adc::OneShot;
use panic_probe as _;
use rp_pico::{self as bsp, hal::adc::AdcPin};

const REFERENCE_VOLTAGE: f32 = 3.3;
const STEPS_12BIT: f32 = 4096_f32;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio22.into_push_pull_output();
    let mut adc = bsp::hal::Adc::new(pac.ADC, &mut pac.RESETS);
    let mut rp2040_temp_sensor = adc.take_temp_sensor().unwrap();
    let mut adc_pin_0 = AdcPin::new(pins.gpio26).unwrap();

    loop {
        // Blink the LED
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();

        // temperture sensor
        let chip_voltage_24bit: u16 = adc.read(&mut rp2040_temp_sensor).unwrap();
        let sensor_voltage_24bit: u16 = adc.read(&mut adc_pin_0).unwrap();

        info!(
            "Temp readings:  Sensor: {}°C, OnChip: {}°C",
            own_t(sensor_voltage_24bit),
            chip_t(chip_voltage_24bit)
        );

        delay.delay_ms(500);
    }
}

fn adc_reading_to_voltage(reading_12bit: u16) -> f32 {
    let voltage = (REFERENCE_VOLTAGE / STEPS_12BIT) * reading_12bit as f32;
    debug!("ADC reading: 0x{:x}, voltage: {}V", reading_12bit, voltage);
    voltage
}

fn own_t(adc_reading: u16) -> f32 {
    let v = adc_reading_to_voltage(adc_reading);
    (100.0 * v) - 50.0
}

fn chip_t(adc_reading: u16) -> f32 {
    let v = adc_reading_to_voltage(adc_reading);
    27.0 - ((v - 0.706) / 0.001721)
}
