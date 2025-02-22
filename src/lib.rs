use glob::glob;
use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::audio::{Sound, load_sound};
use futures::future::join_all;

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

    pub fn empty() -> Self {
        Self {
            images: HashMap::new(),
            sounds: HashMap::new(),
        }
    }

    pub async fn load_image(&self, path: std::path::PathBuf) -> (String, Texture2D) {
        let image = load_texture(path.to_str().unwrap()).await.unwrap();
        image.set_filter(FilterMode::Nearest);
        (path.file_stem().unwrap().to_str().unwrap().to_string(), image)
    }

    pub async fn load_sound(&self, path: std::path::PathBuf) -> (String, Sound) {
        let sound = load_sound(path.to_str().unwrap()).await.unwrap();
        (path.file_stem().unwrap().to_str().unwrap().to_string(), sound)
    }

    pub async fn load_async(&mut self) {
        let mut image_futures = Vec::new();

        for entry in glob("assets/*.png").expect("Failed to load images") {
            match entry {
                Ok(path) => {
                    let future = self.load_image(path);
                    image_futures.push(future);
                },
                Err(e) => println!("{:?}", e),
            }
        }

        for (name, image) in join_all(image_futures).await {
            self.images.insert(name, image);
        }

        let mut sound_futures = Vec::new();

        for entry in glob("assets/sounds/*.wav").expect("Failed to load images") {
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

    pub fn add_get_placeholder(&mut self, text: &str, color: Color, size: Vec2, camera: Option<&Camera2D>) -> &Texture2D {
        let ass_cam = Camera2D {
            render_target: Some(render_target(size.x as u32, size.y as u32)),
            ..Default::default()
        };

        set_camera(&ass_cam);

        clear_background(color);

        draw_text(text, 0., 0., 16., WHITE);

        if let Some(cam) = camera {
            set_camera(cam);
        } else {
            set_default_camera();
        }

        self.images.insert(text.to_string(), ass_cam.render_target.unwrap().texture.clone());

        &self.images[text]
    }
}