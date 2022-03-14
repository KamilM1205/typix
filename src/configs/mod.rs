pub mod layout;
pub mod locale;
pub mod settings;
pub mod theme;

pub use layout::Layout;
pub use locale::Locale;
pub use settings::Settings;
pub use theme::Theme;

use crate::utils::constants::CONFIGS;
use std::io::Write;

pub fn unpack_plugins(
    apath: &std::path::Path,
    ppath: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir: &include_dir::Dir;
    if ppath.to_string_lossy() == "" {
        dir = &CONFIGS;
    } else {
        dir = match CONFIGS.get_dir(ppath) {
            Some(d) => d,
            None => {
                return Err(format!("Path not found: {}", ppath.display()).into());
            }
        };
    }

    for entry in dir.entries().iter() {
        match entry {
            include_dir::DirEntry::File(file) => {
                let mut path = apath.clone().to_path_buf();
                path.push(file.path());
                if !path.exists() {
                    match std::fs::File::create(path) {
                        Ok(mut f) => match write!(f, "{}", file.contents_utf8().unwrap()) {
                            Ok(_) => (),
                            Err(e) => return Err(Box::new(e)),
                        },
                        Err(e) => return Err(Box::new(e)),
                    }
                } else {
                    match std::fs::write(path, file.contents_utf8().unwrap()) {
                        Ok(_) => (),
                        Err(e) => return Err(Box::new(e)),
                    };
                }
            }
            include_dir::DirEntry::Dir(d) => {
                let mut path = apath.clone().to_path_buf();
                path.push(d.path());
                if !path.exists() {
                    match std::fs::create_dir(path) {
                        Ok(_) => (),
                        Err(e) => return Err(Box::new(e)),
                    }
                }
                unpack_plugins(&apath, d.path()).unwrap();
            }
        }
    }
    Ok(())
}

pub fn get_list(path: &std::path::Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        }
        println!("{}", path.display());
        match unpack_plugins(&path, CONFIGS.path()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    let files = match path.read_dir() {
        Ok(e) => Some(e),
        Err(e) => return Err(Box::new(e)),
    };

    let mut file: Vec<String> = Vec::new();
    if let Some(entries) = files {
        for e in entries {
            if let Ok(entry) = e {
                let mut path = entry.path();
                if let Ok(ftype) = entry.file_type() {
                    if ftype.is_dir() == true {
                        path.set_extension("");
                        file.push(String::from(path.file_name().unwrap().to_str().unwrap()));
                    }
                }
            }
        }
    }
    Ok(file)
}
