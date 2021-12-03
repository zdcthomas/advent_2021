use std::{env::current_dir, fs::File, io::Read};

pub fn lines(name: &str) -> Vec<String> {
    let mut path = current_dir().unwrap();
    path.push("input");
    path.push(name);
    dbg!(&path);
    let mut file_contents = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut file_contents)
        .unwrap();

    file_contents.split('\n').map(|s| s.to_owned()).collect()
}
