use serde::Deserialize;

#[derive(Deserialize)]
pub struct Layout {
    pub litterals: String,
    pub numbers: Option<String>,
    pub symbols: Option<String>,
}

impl Layout {
    pub fn load(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().unwrap();
        path.push(format!("typix/configs/layouts/{}", name));
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => return Err(Box::new(e)),
        };

        let decode: Self = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(e) => return Err(Box::new(e)),
        };

        Ok(decode)
    }
}
