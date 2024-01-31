#[cfg(test)]
use std::{fs, path::Path};

#[cfg(test)]
pub fn get_input(file_path: &str, file_name: &str) -> String {
    let mut assets = Path::new(file_path)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    assets.push_str("/assets/");
    assets.push_str(file_name);
    fs::read_to_string(assets).unwrap()
}

#[cfg(test)]
pub fn get_rows(input: String) -> Vec<String> {
    input.split('\n').map(|f| f.to_owned()).collect()
}
