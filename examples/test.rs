extern crate flawnt;

fn main() {
    let config = flawnt::FontConfig::new().unwrap();
    println!("Location: {}", config.get_location().to_string_lossy());
    println!("Font directories: {:#?}", config.get_font_dirs());
    let fonts = config.get_fonts().unwrap();
    for font in fonts {
        if font.to_str().unwrap().ends_with("Sans-Regular.ttf") || font.to_str().unwrap().ends_with("Sans.ttf") {
            println!("{:?}", font);
        }
    }
}
