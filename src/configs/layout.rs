use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Layout {
    pub litterals: String,
    pub numbers: Option<String>,
    pub symbols: Option<String>,
    pub uppercase_litterals: Option<String>,
    #[serde(skip_deserializing)]
    pub list: Vec<String>,
    #[serde(skip_deserializing)]
    pub name: String,
}

impl Layout {
    pub fn load(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().unwrap();
        path.push(format!("typix/configs/layouts/{}", name));
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => return Err(Box::new(e)),
        };

        let mut decode: Self = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(e) => return Err(Box::new(e)),
        };

        path.set_extension("");
        let name = path.file_name().unwrap().to_str().unwrap();
        decode.name = String::from(name);

        decode.list = Layout::get_list();

        Ok(decode)
    }

    pub fn get_list() -> Vec<String> {
        let mut path = dirs::config_dir().unwrap();
        println!("{}", path.display());
        path.push("typix/configs/layouts/");
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
