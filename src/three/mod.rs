use std::str::FromStr;

use super::Day;
#[derive(Debug, PartialEq)]
pub struct Bin {
    inner: Vec<u8>,
}

impl FromStr for Bin {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bins: Vec<u8> = s
            .chars()
            .map(|i| {
                dbg!(&i);
                match i {
                    '1' => 1,
                    '0' => 0,
                    _ => panic!(),
                }
            })
            .collect();
        Ok(Self { inner: bins })
    }
}

impl Bin {
    fn flip_bits(&self) -> Bin {
        let inner: Vec<u8> = self
            .inner
            .iter()
            .map(|bit| match bit {
                1 => 0,
                0 => 1,
                _ => panic!(),
            })
            .collect();
        Bin { inner }
    }
    fn new(inner: Vec<u8>) -> Self {
        Self { inner }
    }

    fn to_neg(&self) -> Vec<i32> {
        self.inner
            .iter()
            .map(|bit| if bit == &0 { -1 } else { 1 })
            .collect()
    }

    fn bin_to_dec(&self) -> i32 {
        let mut num = String::new();
        for bit in &self.inner {
            num.push(if bit == &1 { '1' } else { '0' })
        }
        dbg!(&num);
        i32::from_str_radix(num.as_str(), 2).unwrap()
    }
}

pub struct Three;
impl Day for Three {
    type Input = Vec<Bin>;

    type Return = i32;

    fn parse_file(bin_string: String) -> Self::Input {
        bin_string
            .split('\n')
            .map(|bin| bin.parse::<Bin>().unwrap())
            .collect()
    }

    fn first(lines: Self::Input) -> Self::Return {
        let gamma = most_common(&lines);
        dbg!(&gamma);
        let epsilon = gamma.flip_bits();
        dbg!(&epsilon);
        gamma.bin_to_dec() * epsilon.bin_to_dec()
    }

    fn second(lines: Self::Input) -> Self::Return {
        todo!()
    }
}

fn add(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    if first.is_empty() {
        return second;
    }
    first
        .iter()
        .enumerate()
        .map(|(pos, val)| val + second[pos])
        .collect()
}

fn most_common(lines: &Vec<Bin>) -> Bin {
    // If we change 0 into -1, and add them together, the difference from 0 is the number of 1s or 0s more there are.

    let eval = lines
        .iter()
        .fold(vec![], |acc, line| add(acc, line.to_neg()))
        .iter()
        .map(|bit| if bit > &0 { 1 } else { 0 })
        .collect();
    Bin::new(eval)
}

// sub 0s with negative 1! then if value of sum is 0, the most common is 0, either it's 1.

// gamma rate and epsilon rate
// power = gamma * epsilon
// gamma = most common bit in each position treated at bin
// epsilon = least common bit in each position treated at bin

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn first_day() {
        let bins = Three::parse_file(SAMPLE.to_owned());
        let first = Three::first(bins);
        assert_eq!(198, first);
    }

    #[test]
    fn int_parse_test() {
        let thirty_one = Bin::new(vec![1, 1, 1, 1, 1]);
        let actual = thirty_one.bin_to_dec();
        assert_eq!(31, actual);
    }

    #[test]
    fn full_file() {
        unimplemented!();
    }

    #[test]
    fn add_test() {
        let result = add(vec![1, 2, -1, 0], vec![0, -3, 1, 1]);
        assert_eq!(result, vec![1, -1, 0, 1]);

        let result = add(vec![5, 5, 5, 5], vec![0, -3, -10, 1]);
        assert_eq!(result, vec![5, 2, -5, 6]);
    }

    #[test]
    fn common_bit_test() {
        let bins = Three::parse_file(SAMPLE.to_owned());
        let most_common = most_common(&bins);
        assert_eq!(
            Bin {
                inner: vec![1, 0, 1, 1, 0]
            },
            most_common
        );
        // let lines = vec![
        //     (0, 0, 1, 0, 0),
        //     (1, 1, 1, 1, 0),
        //     (1, 0, 1, 1, 0),
        //     (1, 0, 1, 1, 1),
        //     (1, 0, 1, 0, 1),
        //     (0, 1, 1, 1, 1),
        //     (0, 0, 1, 1, 1),
        //     (1, 1, 1, 0, 0),
        //     (1, 0, 0, 0, 0),
        //     (1, 1, 0, 0, 1),
        //     (0, 0, 0, 1, 0),
        //     (0, 1, 0, 1, 0),
        // ];
        // let common = most_common(&lines);
        // assert_eq!((1, 0, 1, 1, 0), common);
    }
}
