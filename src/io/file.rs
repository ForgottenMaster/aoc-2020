use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

pub fn get_file_contents(path: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
