extern crate quick_xml;
extern crate walkdir;
extern crate xdg;

use std::path::{Path, PathBuf};

use xdg::BaseDirectories;

use quick_xml::events::Event;
use quick_xml::Reader;

use walkdir::WalkDir;

/// Locates fontconfig config
pub fn get_config() -> Option<PathBuf> {
    let xdg_dirs = BaseDirectories::with_prefix("fontconfig").unwrap();
    xdg_dirs.find_config_file("fonts.conf").or_else(|| {
        let config = Path::new("/etc/fonts/fonts.conf");
        match config.exists() {
            true => Some(config.into()),
            false => None,
        }
    })
}

pub fn parse_config(path: &Path) -> Vec<(Vec<String>, String)> {
    let mut reader = Reader::from_file(path).unwrap();
    let reader = reader.trim_text(true);
    let mut buf = Vec::new();
    let mut tracking_tags: Vec<String> = Vec::new();
    let mut data: Vec<(Vec<String>, String)> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(tag)) => {
                let tag = reader.decode(&tag).to_string();
                tracking_tags.push(tag);
            }
            Ok(Event::Text(text)) => {
                let text = reader.decode(&text).to_string();
                data.push((tracking_tags.clone(), text));
            }
            Ok(Event::End(_)) => {
                tracking_tags.pop();
            }
            Err(e) => panic!("Error at position {}", e),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }
    return data;
}

pub struct FontConfig {
    location: PathBuf,
    data: Vec<(Vec<String>, String)>,
}

impl FontConfig {
    pub fn new() -> Result<FontConfig, ()> {
        let location = PathBuf::from(get_config().ok_or_else(|| ())?);
        let data = parse_config(&location);
        Ok(FontConfig { location, data })
    }

    pub fn get_location(&self) -> &Path {
        &self.location
    }

    pub fn get_font_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        for entry in &self.data {
            if entry.0.last() == Some(&"dir".to_string()) {
                let path = PathBuf::from(entry.1.clone());
                if path.exists() {
                    dirs.push(path);
                }
            }
        }
        dirs
    }

    pub fn get_fonts(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut fonts = Vec::new();
        for dir in self.get_font_dirs() {
            for file in WalkDir::new(dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|p| p.file_type().is_file())
            {
                let path = file.into_path();
                if let Some(extension) = path.extension() {
                    match extension.to_str() {
                        Some("ttf") | Some("otf") => fonts.push(path.clone()),
                        _ => {}
                    }
                }
            }
        }
        Ok(fonts)
    }
}
#[cfg(test)]
mod tests {
    use *;
    #[test]
    fn get_directory() {}
}
