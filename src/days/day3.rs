use fancy_regex::Regex;
use std::str::FromStr;
use utils::Solution;

pub struct Day3;

impl Solution for Day3 {
    const DAY: usize = 3;
    fn part1(input: &str) -> i32 {
        let instructions = parse(input);
        let mut sum = 0;

        for instruction in instructions {
            if let Instruction::Mul(x, y) = instruction {
                sum += x * y
            };
        }

        sum
    }

    fn part2(input: &str) -> i32 {
        let instructions = parse(input);
        let mut sum = 0;
        let mut accumulate = true;

        for instruction in instructions {
            match instruction {
                Instruction::Mul(x, y) if accumulate => sum += x * y,
                Instruction::Do => accumulate = true,
                Instruction::Dont => accumulate = false,
                _ => (),
            };
        }

        sum
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
    Unknown,
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mul_regex = r"mul\((?<mul_x>\d{1,3}),(?<mul_y>\d{1,3})\)";
    let do_regex = r"do\(\)";
    let dont_regex = r"don't\(\)";

    let regex = Regex::new(&format!(
        "(?<mul>{mul_regex})\
        |(?<do>{do_regex})\
        |(?<dont>{dont_regex})"
    ))
    .unwrap();

    for cap in regex.captures_iter(input).flatten() {
        let instruction = if cap.name("mul").is_some() {
            let x = i32::from_str(cap.name("mul_x").unwrap().as_str()).unwrap();
            let y = i32::from_str(cap.name("mul_y").unwrap().as_str()).unwrap();
            Instruction::Mul(x, y)
        } else if cap.name("do").is_some() {
            Instruction::Do
        } else if cap.name("dont").is_some() {
            Instruction::Dont
        } else {
            Instruction::Unknown
        };
        instructions.push(instruction);
    }
    instructions
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT_ONLY_MUL: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const SAMPLE_INPUT_COMBINED: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_only_mul() {
        assert_eq!(
            parse(SAMPLE_INPUT_ONLY_MUL),
            vec!(
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            )
        );
    }
    #[test]
    fn test_parse_combined() {
        assert_eq!(
            parse(SAMPLE_INPUT_COMBINED),
            vec!(
                Instruction::Mul(2, 4),
                Instruction::Dont,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Do,
                Instruction::Mul(8, 5)
            )
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day3::part1(SAMPLE_INPUT_ONLY_MUL), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day3::part2(SAMPLE_INPUT_COMBINED), 48);
    }
}
