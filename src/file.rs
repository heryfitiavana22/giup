use std::{fs, path::Path};

use crate::exit;

pub fn is_file_exist(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

pub fn write_file<T, U>(file_path: T, content: U)
where
    T: AsRef<Path> + std::fmt::Debug,
    U: AsRef<[u8]>,
{
    fs::write(&file_path, content).unwrap_or_else(|_| {
        exit!(format!("Unable to write file : {:?}", file_path).as_str());
    });
}

pub fn read_file_to_string<T>(file_path: T) -> String
where
    T: AsRef<Path> + std::fmt::Debug + Clone,
{
    fs::read_to_string(file_path.clone()).unwrap_or_else(|_| {
        exit!(format!("Unable to read file : {:?}", file_path).as_str());
    })
}
