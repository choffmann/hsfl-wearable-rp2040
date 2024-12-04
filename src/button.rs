use embassy_rp::gpio::{Input, Level};
use embassy_time::{Duration, Timer};

pub enum Event {
    ButtonPressed,
}

pub struct DebouncedButton<'a> {
    input: Input<'a>,
    debounce: Duration,
}

impl<'a> DebouncedButton<'a> {
    pub fn new(input: Input<'a>, debounce: Duration) -> Self {
        Self { input, debounce }
    }

    pub async fn debounce(&mut self) -> Level {
        loop {
            let l1 = self.input.get_level();

            self.input.wait_for_any_edge().await;

            Timer::after(self.debounce).await;

            let l2 = self.input.get_level();
            if l1 != l2 {
                break l2;
            }
        }
    }
}
