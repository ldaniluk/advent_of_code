use itertools::Itertools;
use std::str::FromStr;
use utils::Solution;

pub struct Day2;

impl Solution for Day2 {
    const DAY: usize = 2;
    fn part1(input: &str) -> i32 {
        let reports = split_input_into_list_of_lists(input);

        reports.iter().filter(|rep| is_safe(rep)).count() as i32
    }

    fn part2(input: &str) -> i32 {
        let reports = split_input_into_list_of_lists(input);

        reports
            .iter()
            .filter(|rep| is_safe(rep) || is_subset_safe(rep))
            .count() as i32
    }
}

fn is_subset_safe(report: &[i32]) -> bool {
    (0..report.len()).any(|skip_offset| is_safe(&complement(report, skip_offset)))
}

fn complement(report: &[i32], n: usize) -> Vec<i32> {
    report
        .iter()
        .enumerate()
        .filter_map(|(idx, el)| (idx != n).then_some(*el))
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let diff_range = 1..=3;

    let is_increasing = report
        .iter()
        .tuple_windows()
        .all(|(e1, e2)| diff_range.contains(&(e2 - e1)));

    if is_increasing {
        return true;
    }

    let is_decreasing = report
        .iter()
        .tuple_windows()
        .all(|(e1, e2)| diff_range.contains(&(e1 - e2)));

    is_decreasing
}

fn split_input_into_list_of_lists(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|el| i32::from_str(el).expect("nan in input"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    #[test]
    fn test_complement() {
        assert_eq!(complement(&[1, 2, 3], 1), vec!(1, 3));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day2::part1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day2::part2(SAMPLE_INPUT), 4);
    }
}
