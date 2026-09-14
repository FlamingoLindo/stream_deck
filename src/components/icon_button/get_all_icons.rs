use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_icons_paths() -> Vec<PathBuf> {
    let dir = Path::new("./assets/icons");
    let mut icons = vec![];

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("failed to read icons dir: {e}");
            return icons;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("failed to read dir entry: {e}");
                continue;
            }
        };
        icons.push(entry.path());
    }

    icons
}
