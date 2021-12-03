use super::Day;

pub struct One;

impl Day for One {
    type Return = i32;
    type Input = Vec<i32>;

    fn first(lines: Vec<i32>) -> i32 {
        lines
            .windows(2)
            .fold(0, |acc, pair| if pair[1] > pair[0] { acc + 1 } else { acc })
    }

    fn second(lines: Vec<i32>) -> i32 {
        let mut iter = lines.windows(3);
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

    fn parse_file(file_string: String) -> Self::Input {
        file_string
            .split('\n')
            .filter_map(|el| el.parse::<i32>().ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_value() -> Vec<i32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_value_gives_correct_answer() {
        assert_eq!(One::first(test_value()), 7);
    }

    #[test]
    fn triple_sums() {
        assert_eq!(One::second(test_value()), 5);
    }
}
