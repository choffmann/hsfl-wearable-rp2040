#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU8, Ordering};

use button::DebouncedButton;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pin, Pull};
use embassy_time::{with_timeout, Duration, Instant, Timer};
use panic_probe as _;
use state::StateManager;

pub mod button;
pub mod state;

static SHARED_STATE: AtomicU8 = AtomicU8::new(0);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program start");
    let p = embassy_rp::init(Default::default());
    let mut state_manager = StateManager::new();
    let mut button = DebouncedButton::new(Input::new(p.PIN_3, Pull::Up), Duration::from_millis(50));

    unwrap!(spawner.spawn(led_task((
        p.PIN_22.degrade(),
        p.PIN_21.degrade(),
        p.PIN_20.degrade()
    ),)));

    loop {
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
