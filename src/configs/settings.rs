use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub fullscreen: bool,
    pub samples: i32,
    pub window_width: i32,
    pub window_height: i32,
    pub locale: String,
    #[serde(skip_serializing, skip_deserializing)]
    path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            fullscreen: false,
            samples: 4,
            window_width: 640,
            window_height: 480,
            locale: "en".to_owned(),
            path: String::new(),
        }
    }
}

impl Settings {
    pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => return Err(Box::new(e)),
        };

        let mut decode: Self = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(e) => return Err(Box::new(e)),
        };

        decode.path = String::from(path.to_str().unwrap());

        Ok(decode)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new(&self.path);
        let json = match serde_json::to_string(self) {
            Ok(content) => content,
            Err(e) => return Err(Box::new(e)),
        };

        let mut file = match std::fs::File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(Box::new(e)),
        };
        match file.write_all(json.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };

        println!("Settings saved width: {}", json);

        Ok(())
    }

    pub fn set_path(&mut self, path: &std::path::Path) {
        self.path = String::from(path.to_str().unwrap());
    }
}
