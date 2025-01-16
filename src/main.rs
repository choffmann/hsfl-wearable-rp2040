#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU8, Ordering};

use button::DebouncedButton;
use defmt::*;
use defmt_rtt as _;
use embassy_dht::dht11::DHT11;
use embassy_executor::Spawner;
use embassy_rp::{
    adc::{Adc, Channel, Config, InterruptHandler},
    bind_interrupts,
    gpio::{AnyPin, Input, Level, Output, Pin, Pull},
};
use embassy_time::{with_timeout, Delay, Duration, Instant, Timer};
use panic_probe as _;
use state::StateManager;

pub mod button;
pub mod state;

static SHARED_STATE: AtomicU8 = AtomicU8::new(0);
const REFERENCE_VOLTAGE: f32 = 3.3;
const STEPS_12BIT: f32 = 4096_f32;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program start");
    let p = embassy_rp::init(Default::default());
    let mut state_manager = StateManager::new();
    let mut button =
        DebouncedButton::new(Input::new(p.PIN_21, Pull::Up), Duration::from_millis(50));

    unwrap!(spawner.spawn(led_task((
        p.PIN_16.degrade(),
        p.PIN_17.degrade(),
        p.PIN_19.degrade()
    ),)));

    unwrap!(spawner.spawn(transistor_task(p.PIN_7.degrade())));
    unwrap!(spawner.spawn(temp_task(p.PIN_15.degrade())));

    let mut adc = Adc::new(p.ADC, Irqs, Config::default());
    let mut temp_sensor = Channel::new_pin(p.PIN_26, Pull::None);

    loop {
        let temp = adc.read(&mut temp_sensor).await.unwrap();
        info!("Raw temp: {}", temp);
        info!("Temp: {} degrees", own_t(temp));

        button.debounce().await;
        let start = Instant::now();
        info!("Button pressed");

        match with_timeout(Duration::from_secs(1), button.debounce()).await {
            Ok(_) => {
                info!("Button pressed for: {}ms", Instant::now() - start);
                state_manager.next();
            }
            Err(_) => {
                info!("Timeout");
                state_manager.set_off();
            }
        }

        SHARED_STATE.store(state_manager.state.clone().into(), Ordering::Relaxed);
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

#[embassy_executor::task]
async fn led_task(led: (AnyPin, AnyPin, AnyPin)) {
    let (mut led1, mut led2, mut led3) = (
        Output::new(led.0, Level::Low),
        Output::new(led.1, Level::Low),
        Output::new(led.2, Level::Low),
    );
    loop {
        let state: state::State = SHARED_STATE.load(Ordering::Relaxed).into();
        match state {
            state::State::Off => {
                led1.set_low();
                led2.set_low();
                led3.set_low();
            }
            state::State::Low => {
                led1.set_high();
                led2.set_low();
                led3.set_low();
            }
            state::State::Mid => {
                led1.set_high();
                led2.set_high();
                led3.set_low();
            }
            state::State::High => {
                led1.set_high();
                led2.set_high();
                led3.set_high();
            }
            state::State::Auto => {
                info!("Auto mode");
                led1.set_high();
                led2.set_low();
                led3.set_high();
                Timer::after(Duration::from_millis(500)).await;
                led1.set_low();
                Timer::after(Duration::from_millis(500)).await;
                led2.set_low();
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn transistor_task(pin: AnyPin) {
    debug!("Transistor task");
    let mut pin = Output::new(pin, Level::Low);
    loop {
        let state: state::State = SHARED_STATE.load(Ordering::Relaxed).into();
        match state {
            state::State::Off => {
                pin.set_low();
            }
            state::State::Low => {
                pin.set_high();
                info!("state: {:?}, transistor: high", state);
                Timer::after(Duration::from_millis(500)).await;
                pin.set_low();
                info!("state: {:?}, transistor: low", state);
                Timer::after(Duration::from_secs(1)).await;
            }
            state::State::Mid => {
                pin.set_high();
                info!("state: {:?}, transistor: high", state);
                Timer::after(Duration::from_millis(700)).await;
                pin.set_low();
                info!("state: {:?}, transistor: low", state);
                Timer::after(Duration::from_millis(300)).await;
            }
            state::State::High => {
                pin.set_high();
                info!("state: {:?}, transistor: high", state);
            }
            state::State::Auto => {
                pin.set_low();
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn temp_task(pin: AnyPin) {
    let mut temp_sens = DHT11::new(pin, Delay);
    loop {
        match temp_sens.read() {
            Ok(reading) => {
                let temp = reading.get_temp();
                let hum = reading.get_hum();
                info!("Temp: {}°C, Humi: {}", temp, hum);
            }
            Err(e) => {
                warn!("Error reading temp: {:?}", e);
            }
        }
        Timer::after(Duration::from_millis(200)).await;
    }
}
