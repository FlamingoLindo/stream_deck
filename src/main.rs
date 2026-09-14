use crate::{
    components::icon_button::get_all_icons::get_icons_paths,
    settings::settings::{BtnAction, BtnType, DeckSettings},
};
use slint::{Color, Image, ModelRc, Timer, TimerMode, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_F13, VK_F14, VK_F15, VK_F16, VK_F17};

pub mod components;
pub mod handler;
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
    let deck_settings = Rc::new(RefCell::new(DeckSettings::load_settings()));
    let icons = get_icons_paths();

    let (actions, button_data) = {
        let settings = deck_settings.borrow();
        let tab = settings
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

        (actions, button_data)
    };

    let actions = Rc::new(RefCell::new(actions));

    let buttons_vec_model = Rc::new(VecModel::from(button_data));
    let buttons_model = ModelRc::from(buttons_vec_model.clone());

    let icon_images: Vec<Image> = icons
        .iter()
        .filter_map(|path| match Image::load_from_path(path) {
            Ok(img) => Some(img),
            Err(e) => {
                eprintln!("failed to load icon {path:?}: {e}");
                None
            }
        })
        .collect();

    let icons_model = ModelRc::new(VecModel::from(icon_images));

    let window = MainWindow::new().unwrap();
    window.set_buttons(buttons_model);
    window.set_icons(icons_model);

    let weak = window.as_weak();

    let weak_for_timer = weak.clone();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
        if let Some(window) = weak_for_timer.upgrade() {
            window.set_current_time(components::clock::clock_logic::get_realtime().into());
        }
    });

    let actions_for_key = actions.clone();
    let weak_for_key = weak.clone();
    window.on_key_pressed(move |id| {
        let actions = actions_for_key.borrow();
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
                    if let Some(window) = weak_for_key.upgrade() {
                        window.set_current_page(0);
                    }
                }
                "add" => {
                    if let Some(window) = weak_for_key.upgrade() {
                        window.set_current_page(2);
                    }
                }
                other => eprintln!("unknown nav target: {other}"),
            },
        }
    });

    let deck_settings_for_add = deck_settings.clone();
    let icons_for_add = icons.clone();
    let actions_for_add = actions.clone();
    let weak_for_add = weak.clone();

    window.on_icon_selected(move |i| {
        if handler::handle_icon_selected(
            i,
            &icons_for_add,
            &deck_settings_for_add,
            &actions_for_add,
            &buttons_vec_model,
        )
        .is_some()
        {
            if let Some(window) = weak_for_add.upgrade() {
                window.set_current_page(1);
            }
        }
    });

    window.run().unwrap();
}
