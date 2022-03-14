use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct Locale {
    pub locale: HashMap<String, String>,
    #[serde(skip_deserializing)]
    pub list: Vec<String>,
}

impl Locale {
    pub fn load(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().unwrap();
        path.push(format!("typix/configs/locales/{}", name));
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => return Err(Box::new(e)),
        };

        let mut decode: Locale = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(e) => return Err(Box::new(e)),
        };

        decode.list = Locale::get_list();

        Ok(decode)
    }

    fn get_list() -> Vec<String> {
        let mut path = dirs::config_dir().unwrap();
        println!("{}", path.display());
        path.push("typix/configs/locales/");
        println!("{}", path.display());
        let files = std::fs::read_dir(path).unwrap();

        let mut list: Vec<String> = Vec::new();

        for file in files {
            let file = file.unwrap();
            if file.file_type().unwrap().is_file() {
                list.push(String::from(
                    file.path()
                        .with_extension("")
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                ));
            }
        }

        list
    }
}
