use std::iter::zip;
use std::str::FromStr;
use utils::Solution;

pub struct Day1;

impl Solution for Day1 {
    const DAY: usize = 1;

    fn part1(input: &str) -> i32 {
        let (mut left, mut right) = split_input_into_two_lists(input);

        left.sort();
        right.sort();

        zip(left.iter(), right.iter())
            .map(|(left_value, right_value)| (left_value - right_value).abs())
            .sum::<i32>()
    }

    fn part2(input: &str) -> i32 {
        let (left, right) = split_input_into_two_lists(input);

        left.iter()
            .map(|num| right.iter().filter(|x| *x == num).count() as i32 * num)
            .sum::<i32>()
    }
}

fn split_input_into_two_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input.split_whitespace().enumerate().for_each(|(idx, txt)| {
        let num = i32::from_str(txt).expect("nan in input");
        if idx % 2 == 0 {
            left.push(num)
        } else {
            right.push(num)
        }
    });

    (left, right)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        assert_eq!(Day1::part1(SAMPLE_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day1::part2(SAMPLE_INPUT), 31);
    }
}
