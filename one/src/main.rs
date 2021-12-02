use std::fs::{self, File};
use std::io::{Error, Read};
use std::path::PathBuf;

fn main() {
    println!("First: {}\nSecond: {}", first(), second());
}

fn count_increases(values: Vec<i32>) -> i32 {
    values
        .windows(2)
        .fold(0, |acc, pair| if pair[1] > pair[0] { acc + 1 } else { acc })
}

fn first() -> i32 {
    let ints = ints_from_file("./input/first.txt");
    count_increases(ints)
}

fn count_increases_in_threes(values: Vec<i32>) -> i32 {
    let mut iter = values.windows(3);
    let mut prev: &[i32] = iter.next().unwrap();
    let mut count = 0;

    for window in iter {
        if window.iter().sum::<i32>() > prev.iter().sum::<i32>() {
            count = count + 1
        }
        prev = window;
    }
    count
}

fn ints_from_file(name: &str) -> Vec<i32> {
    let path = PathBuf::from("./input/first.txt");
    let mut file_contents = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut file_contents)
        .unwrap();

    file_contents
        .split('\n')
        .filter_map(|el| el.parse::<i32>().ok())
        .collect()
}

fn second() -> i32 {
    let ints = ints_from_file("./input/first.txt");
    count_increases_in_threes(ints)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_value() -> Vec<i32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_value_gives_correct_answer() {
        assert_eq!(count_increases(test_value()), 7);
    }
    #[test]
    fn real_path() {
        let result = first();
    }

    #[test]
    fn triple_sums() {
        assert_eq!(count_increases_in_threes(test_value()), 5);
    }
}
