use std::fs::read_to_string;
use std::path::PathBuf;

pub fn bin_dir_file(file_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = std::env::current_exe()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let file_split: Vec<&str> = file_name.split(".").collect();

    match dir.rfind("/") {
        Some(e) => {
            let mut buf = PathBuf::new();
            buf.push(dir[0..e].to_string());
            buf.push(file_split[0]);
            buf.set_extension(file_split[1]);
            Ok(buf)
        }
        None => Err("Failed to format dir")?,
    }
}

pub fn read_lines(filename: PathBuf) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
