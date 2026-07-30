use slint::{Color, Image, ModelRc, Timer, TimerMode, VecModel};
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_F13, VK_F14, VK_F15, VK_F16, VK_F17};

use crate::settings::load::{BtnAction, BtnType, load_settings};

pub mod components;
pub mod settings;

fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Color::from_rgb_u8(r, g, b)
}

slint::include_modules!();

fn main() {
    let deck_settings = load_settings();

    let tab = deck_settings
        .tabs
        .get("tab1")
        .expect("tab1 not found in settings");

    let actions: Vec<BtnAction> = tab.buttons.iter().map(|b| b.action.clone()).collect();

    let button_data: Vec<ButtonData> = tab
        .buttons
        .iter()
        .map(|b| ButtonData {
            image: Image::load_from_path(std::path::Path::new(&b.image)).unwrap_or_default(),
            color: parse_hex_color(&b.color),
        })
        .collect();

    let buttons_model = ModelRc::new(VecModel::from(button_data));

    let window = MainWindow::new().unwrap();

    window.set_buttons(buttons_model);

    let weak = window.as_weak();

    let weak_for_timer = weak.clone();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
        if let Some(window) = weak_for_timer.upgrade() {
            window.set_current_time(components::clock::clock_logic::get_realtime().into());
        }
    });

    window.on_key_pressed(move |id| {
        let action = match actions.get(id as usize) {
            Some(a) => a,
            None => {
                eprintln!("no action bound for button id {id}");
                return;
            }
        };

        match action.btn_type {
            BtnType::Key => {
                let vk = match action.value.as_str() {
                    "F13" => VK_F13,
                    "F14" => VK_F14,
                    "F15" => VK_F15,
                    "F16" => VK_F16,
                    "F17" => VK_F17,
                    other => {
                        eprintln!("unknown key value: {other}");
                        return;
                    }
                };
                components::icon_button::virtual_keys::send_key(vk);
            }
            BtnType::Nav => match action.value.as_str() {
                "back" => {
                    if let Some(window) = weak.upgrade() {
                        window.set_current_page(0);
                    }
                }
                "add" => {
                    if let Some(window) = weak.upgrade() {
                        window.set_current_page(2);
                    }
                }
                other => eprintln!("unknown nav target: {other}"),
            },
        }
    });

    window.run().unwrap();
}
