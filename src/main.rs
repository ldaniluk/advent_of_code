mod days;

use std::fs::File;
use std::io::Read;
use utils::Solution;

use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;

fn main() {
    run_day::<Day1>();
    run_day::<Day2>();
    run_day::<Day3>();
    run_day::<Day4>();
}

fn get_input(day: usize) -> String {
    let mut input_file = File::open(format!("data/day{day}.in")).expect("file not found");
    let mut buf = String::new();
    input_file
        .read_to_string(&mut buf)
        .expect("failed to read day data");
    buf
}

fn run_day<Day>()
where
    Day: Solution,
{
    let day = Day::DAY;
    let input = get_input(day);

    let part1_result = Day::part1(&input);
    println!("Day {day} part 1: {part1_result:?}");

    let part2_result = Day::part2(&input);
    println!("Day {day} part 2: {part2_result:?}")
}
