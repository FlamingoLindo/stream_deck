use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs};

#[derive(Serialize, Deserialize)]
pub struct DeckSettings {
    version: String,
    #[serde(flatten)]
    pub tabs: BTreeMap<String, Tab>,
}

#[derive(Serialize, Deserialize)]
pub struct Tab {
    pub buttons: Vec<TabBtn>,
}

#[derive(Serialize, Deserialize)]
pub struct TabBtn {
    pub id: i8,
    pub image: String,
    pub color: String,
    pub action: BtnAction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BtnType {
    Key,
    Nav,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BtnAction {
    pub btn_type: BtnType,
    pub value: String,
}

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

impl Default for DeckSettings {
    fn default() -> Self {
        let mut tabs = BTreeMap::new();

        tabs.insert(
            "tab1".to_string(),
            Tab {
                buttons: vec![
                    TabBtn {
                        id: 0,
                        image: "assets/icons/uil--clock.svg".to_string(),
                        color: "#4287f5".to_string(),
                        action: BtnAction {
                            btn_type: BtnType::Nav,
                            value: "back".to_string(),
                        },
                    },
                    TabBtn {
                        id: 1,
                        image: "assets/icons/ic--baseline-plus.svg".to_string(),
                        color: "#4287f5".to_string(),
                        action: BtnAction {
                            btn_type: BtnType::Nav,
                            value: "add".to_string(),
                        },
                    },
                ],
            },
        );

        DeckSettings {
            version: get_version(),
            tabs,
        }
    }
}

pub fn load_settings() -> DeckSettings {
    if !fs::exists("settings.toml").unwrap_or(false) {
        let default_settings = DeckSettings::default();
        let toml_string = toml::to_string_pretty(&default_settings)
            .expect("failed to serialize default settings");
        fs::write("settings.toml", toml_string).expect("failed to write settings.toml");
        default_settings
    } else {
        let contents = fs::read_to_string("settings.toml").expect("failed to read settings.toml");
        toml::from_str(&contents).expect("failed to parse settings.toml")
    }
}
