use std::path::PathBuf;

pub fn get_mime_by_filename(filename: &PathBuf) -> Option<String> {
    let guess = mime_guess::from_path(filename);
    let mime = guess.first();

    if mime.is_some() {
        let mime = mime.unwrap();
        return Some(mime.to_string());
    } else {
        return None;
    }
}
