pub trait Solution {
    const DAY: usize;
    fn part1(input: &str) -> i32;
    fn part2(input: &str) -> i32;
}

#[macro_export]
macro_rules! create_tests {
    ($day:ident, $sample_input:literal) => {
        #[test]
        fn test_part1() {
            assert_eq!($day::part1($sample_input), 11);
        }

        #[test]
        fn test_part2() {
            assert_eq!($day::part2($sample_input), 31);
        }
    };
}
