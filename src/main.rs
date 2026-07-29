use slint::{Timer, TimerMode};
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_F13, VK_F14, VK_F15, VK_F16, VK_F17};

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

    window.on_key_pressed(move |id| {
        let vk = match id {
            2 => VK_F13,
            3 => VK_F14,
            4 => VK_F15,
            5 => VK_F16,
            6 => VK_F17,
            _ => return,
        };
        components::icon_button::virtual_keys::send_key(vk);
    });

    window.run().unwrap();
}
