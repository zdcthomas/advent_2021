use std::fmt::Debug;
mod helper;
mod one;
mod two;

pub trait Day {
    type Input;
    type Return;

    fn parse_file(file_string: String) -> Self::Input;
    fn first(lines: Self::Input) -> Self::Return;
    fn second(lines: Self::Input) -> Self::Return;
}

fn main() {
    // let first = one::first();
    // let second = one::second();
    // println!("First: {}\nSecond: {}", first, second);
}
