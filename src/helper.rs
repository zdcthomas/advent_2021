use std::{env::current_dir, fs::File, io::Read, path::PathBuf};

pub fn input_file(name: &str) -> PathBuf {
    let mut current_dir = current_dir().unwrap();
    current_dir.push("input");
    current_dir.push(name);
    current_dir
}

pub fn input_file_string(name: &str) -> String {
    std::fs::read_to_string(input_file(name))
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .to_owned()
}

pub fn lines(name: &str) -> Vec<String> {
    let mut path = current_dir().unwrap();
    path.push("input");
    path.push(name);
    let mut file_contents = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut file_contents)
        .unwrap();

    file_contents.split('\n').map(|s| s.to_owned()).collect()
}
