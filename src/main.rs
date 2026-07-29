use std::time::Duration;

use slint::{Timer, TimerMode};

pub mod components;

slint::include_modules!();

fn main() {
    let window = MainWindow::new().unwrap();

    let weak = window.as_weak();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
        if let Some(window) = weak.upgrade() {
            window.set_current_time(components::clock::clock_logic::get_realtime().into());
        }
    });

    window.run().unwrap();
}
