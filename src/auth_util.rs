use std::{fs, path::Path};

fn get_auth_as_bytes() -> &'static [u8] {
    let file_contents: &'static [u8] = include_bytes!("SWMAuth2");
    return file_contents;
}

pub fn save_swm_auth() {
    let file_contents: &'static [u8] = get_auth_as_bytes();
    let path = Path::new("SWMAuth2");
    let display = path.display();

    if let Err(why) = fs::write(path, file_contents) {
        panic!("couldn't write SWMAuth2 to {}: {}", display, why);
    };
}
