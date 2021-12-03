use super::Day;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};
pub struct Two;

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(format!("{:?} is an invalid direction", s)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn nav(&mut self, command: String) {
        let (direction, amount) = direction_and_amount(command);
        match direction {
            Direction::Forward => self.horizontal = self.horizontal + amount,
            Direction::Down => self.depth = self.depth + amount,
            Direction::Up => self.depth = self.depth - amount,
        }
    }

    fn nav_and_aim(&mut self, command: String) {
        let (direction, amount) = direction_and_amount(command);
        match direction {
            Direction::Down => self.aim = self.aim + amount,
            Direction::Up => self.aim = self.aim - amount,
            Direction::Forward => {
                self.horizontal = self.horizontal + amount;
                self.depth = self.depth + (self.aim * amount);
            }
        }
    }

    fn nav_from_list(&mut self, command_list: Vec<String>) {
        for command in command_list {
            self.nav(command)
        }
    }

    fn nav_and_aim_from_list(&mut self, command_list: Vec<String>) {
        for command in command_list {
            dbg!(&self);
            self.nav_and_aim(command)
        }
    }

    fn final_product(&self) -> i32 {
        self.horizontal * self.depth
    }
}

fn direction_and_amount(command: String) -> (Direction, i32) {
    let mut iter = command.split(' ');
    let direction: Direction = iter
        .next()
        .expect("couldn't get next iter")
        .parse()
        .expect("couldn't parse direction");
    let amount: i32 = iter.next().unwrap().parse().unwrap();
    (direction, amount)
}

impl Day for Two {
    type Input = Vec<String>;

    type Return = i32;

    fn parse_file(file_string: String) -> Self::Input {
        file_string
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect()
    }

    fn first(lines: Self::Input) -> Self::Return {
        let mut position = Position::new();
        position.nav_from_list(lines);
        position.final_product()
    }

    fn second(lines: Self::Input) -> Self::Return {
        let mut position = Position::new();
        position.nav_and_aim_from_list(lines);
        position.final_product()
    }
}

#[cfg(test)]
mod tests {

    fn test_val() -> Vec<String> {
        [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .to_vec()
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    use super::*;
    #[test]
    fn test_nav() {
        let mut position = Position::new();
        position.nav_from_list(test_val());
        assert_eq!(
            position,
            Position {
                horizontal: 15,
                depth: 10,
                aim: 0,
            }
        );

        assert_eq!(position.final_product(), 150);
    }

    #[test]
    fn test_nav_and_aim() {
        let mut position = Position::new();
        position.nav_and_aim_from_list(test_val());

        assert_eq!(
            position,
            Position {
                horizontal: 15,
                depth: 60,
                aim: 10,
            }
        );

        assert_eq!(position.final_product(), 900);
    }
}
