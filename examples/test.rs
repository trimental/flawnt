extern crate flawnt;
use std::io::prelude::*;

fn main() {
    let config = flawnt::FontConfig::new().unwrap();
    println!("Location: {}", config.get_location().to_string_lossy());
    println!("Font directories: {:#?}", config.get_font_dirs());
    let fonts_dir_files = config.get_font_dir_files().unwrap();
    // println!("{:#?}", fonts_dir_files);
    let mut fonts: Vec<String> = Vec::new();
    for dir in fonts_dir_files {
        let mut file = ::std::fs::File::open(dir).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf);

        for line in buf.lines().filter(|l| l.find("medium-r-normal").is_some()) {
            if let Some(split) = line.find(" ") {
                let name = line[..split].to_string();
                let settings = line[split..].to_string();
                let mut char_buf = String::new();
                for c in line.chars() {
                    if c == ' ' || c == '-' {
                        char_buf.clear()
                    } else {
                        char_buf.push(c);
                        if char_buf == "sans" {
                            if !fonts.contains(&name) {
                                fonts.push(name);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    for font in &fonts {
        // if font.find("-").is_none() {
        println!("{:#?}", font);
        // }
    }
}
