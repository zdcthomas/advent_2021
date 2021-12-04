use std::fmt::Debug;
mod helper;

pub trait Day {
    type Input;
    type Return: Debug;

    fn parse_file(file_string: String) -> Self::Input;
    fn first(lines: Self::Input) -> Self::Return;
    fn second(lines: Self::Input) -> Self::Return;
}

fn main() {
    // let first = one::first();
    // let second = one::second();
    // println!("First: {}\nSecond: {}", first, second);
}

// AUTO GEN

mod one;
mod three;
mod two;
