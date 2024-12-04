use glob::glob;
use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::audio::{Sound, load_sound};

#[derive(Debug)]
pub struct AssetManager {
    pub images: HashMap<String, Texture2D>,
    pub sounds: HashMap<String, Sound>
}

impl AssetManager {
    pub async fn new() -> Self {
        let mut manager = Self {
            images: HashMap::new(),
            sounds: HashMap::new(),
        };

        for entry in glob("assets/*.png").expect("Failed to load images") {
            match entry {
                Ok(path) => {
                    let tex = load_texture(path.to_str().unwrap()).await.unwrap();
                    tex.set_filter(FilterMode::Nearest);
                    manager.images.insert(path.file_stem().unwrap().to_str().unwrap().to_string(), tex);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        for entry in glob("assets/sounds/*.wav").expect("Failed to load images") {
            match entry {
                Ok(path) => {
                    let sound = load_sound(path.to_str().unwrap()).await.unwrap();
                    //tex.set_filter(FilterMode::Nearest);
                    manager.sounds.insert(path.file_stem().unwrap().to_str().unwrap().to_string(), sound);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        manager
    }
}