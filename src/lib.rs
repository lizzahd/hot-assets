use glob::glob;
use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::audio::{Sound, load_sound};
use futures::future::join_all;

#[derive(Debug)]
pub struct AssetManager {
    pub images: HashMap<String, Image>,
    pub textures: HashMap<String, Texture2D>,
    pub sounds: HashMap<String, Sound>,
}

impl AssetManager {
    pub async fn new(textures_folder: Option<&str>, images_folder: Option<&str>, sounds_folder: Option<&str>) -> Self {
        let mut manager = Self {
            images: HashMap::new(),
            textures: HashMap::new(),
            sounds: HashMap::new(),
        };

        if let Some(dir) = textures_folder {
            for entry in glob(&format!("{}/*.png", dir)).expect("Failed to load textures") {
                match entry {
                    Ok(path) => {
                        let tex = load_texture(path.to_str().unwrap()).await.unwrap();
                        tex.set_filter(FilterMode::Nearest);
                        manager.textures.insert(path.file_stem().unwrap().to_str().unwrap().to_string(), tex);
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
        }

        if let Some(dir) = images_folder {
            for entry in glob(&format!("{}/*.png", dir)).expect("Failed to load textures") {
                match entry {
                    Ok(path) => {
                        let tex = load_image(path.to_str().unwrap()).await.unwrap();
                        // tex.set_filter(FilterMode::Nearest);
                        manager.images.insert(path.file_stem().unwrap().to_str().unwrap().to_string(), tex);
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
        }

        if let Some(dir) = sounds_folder {
            for entry in glob(&format!("{}/*.wav", dir)).expect("Failed to load textures") {
                match entry {
                    Ok(path) => {
                        let sound = load_sound(path.to_str().unwrap()).await.unwrap();
                        manager.sounds.insert(path.file_stem().unwrap().to_str().unwrap().to_string(), sound);
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
        }

        manager
    }

    pub fn empty() -> Self {
        Self {
            images: HashMap::new(),
            textures: HashMap::new(),
            sounds: HashMap::new(),
        }
    }

    pub fn image_to_tex(&mut self, name: &str) {
        self.textures.insert(name.to_string(), Texture2D::from_image(&self.images[name]));
    }

    pub fn tex_to_image(&mut self, name: &str) {
        self.images.insert(name.to_string(), self.textures[name].get_texture_data());
    }

    pub async fn load_texture(&self, path: std::path::PathBuf) -> (String, Texture2D) {
        let tex = load_texture(path.to_str().unwrap()).await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        (path.file_stem().unwrap().to_str().unwrap().to_string(), tex)
    }

    pub async fn load_image(&self, path: std::path::PathBuf) -> (String, Image) {
        let image = load_image(path.to_str().unwrap()).await.unwrap();
        (path.file_stem().unwrap().to_str().unwrap().to_string(), image)
    }

    pub async fn load_sound(&self, path: std::path::PathBuf) -> (String, Sound) {
        let sound = load_sound(path.to_str().unwrap()).await.unwrap();
        (path.file_stem().unwrap().to_str().unwrap().to_string(), sound)
    }

    pub async fn load_async(&mut self) {
        // TODO: remove glob, read directory asynchronously
        let mut image_futures = Vec::new();

        for entry in glob("assets/*.png").expect("Failed to load textures") {
            match entry {
                Ok(path) => {
                    let future = self.load_texture(path);
                    image_futures.push(future);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        for (name, image) in join_all(image_futures).await {
            self.textures.insert(name, image);
        }

        let mut sound_futures = Vec::new();

        for entry in glob("assets/sounds/*.wav").expect("Failed to load textures") {
            match entry {
                Ok(path) => {
                    let future = self.load_sound(path);
                    sound_futures.push(future);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        for (name, sound) in join_all(sound_futures).await {
            self.sounds.insert(name, sound);
        }
    }

    pub async fn new_async() -> Self {
        let mut manager = Self::empty();
        manager.load_async().await;
        manager
    }
}