use crate::settings::settings::{BtnAction, BtnType, DeckSettings, TabBtn};
use slint::{Image, VecModel};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::{ButtonData, parse_hex_color};

pub fn handle_icon_selected(
    icon_index: i32,
    icons: &[PathBuf],
    settings: &Rc<RefCell<DeckSettings>>,
    actions: &Rc<RefCell<Vec<BtnAction>>>,
    buttons_model: &Rc<VecModel<ButtonData>>,
) -> Option<TabBtn> {
    let icon_path = icons.get(icon_index as usize)?;

    let mut settings_ref = settings.borrow_mut();
    let next_id = settings_ref
        .tabs
        .get("tab1")
        .map(|t| t.buttons.len() as i8)
        .unwrap_or(0);

    let new_btn = TabBtn {
        id: next_id,
        image: icon_path.to_string_lossy().into_owned(),
        color: "#4287f5".to_string(),
        action: BtnAction {
            btn_type: BtnType::Key,
            value: "F13".to_string(),
        },
    };

    settings_ref.add_btn("tab1", new_btn.clone());
    drop(settings_ref);

    actions.borrow_mut().push(new_btn.action.clone());

    buttons_model.push(ButtonData {
        image: Image::load_from_path(std::path::Path::new(&new_btn.image)).unwrap_or_default(),
        color: parse_hex_color(&new_btn.color),
    });

    Some(new_btn)
}
