use itertools::Itertools;
use std::collections::HashMap;
use utils::Solution;

pub struct Day4;

impl Solution for Day4 {
    const DAY: usize = 4;

    fn part1(input: &str) -> i32 {
        let input_board = Board::from_str(input);

        let xmas_straight = Board::from_str("XMAS");
        let xmas_diagonal = Board::from_str(
            "X...\n\
            .M..\n\
            ..A.\n\
            ...S\n",
        );

        // other valid XMAS writings can be constructed by performing 90deg rotations
        // there's need for only 3 rotations, since 4th one should get us back to the starting board
        let mut boards = Vec::new();
        for rotation_count in 0..=3 {
            boards.push(xmas_straight.rotate_right_many_times(rotation_count));
            boards.push(xmas_diagonal.rotate_right_many_times(rotation_count));
        }

        boards
            .iter()
            .map(|xmas_board| input_board.find_overlaps(xmas_board, Some('.')).len() as i32)
            .sum()
    }

    fn part2(input: &str) -> i32 {
        let input_board = Board::from_str(input);

        let mas_board = Board::from_str(
            "M..\n\
            .A.\n\
            ..S",
        );
        let mas_left_boards = [mas_board.clone(), mas_board.rotate_right_many_times(2)];
        let mas_right_boards = [
            mas_board.rotate_right(),
            mas_board.rotate_right_many_times(3),
        ];

        let valid_xmas_boards = mas_left_boards
            .iter()
            .cartesian_product(mas_right_boards)
            .filter_map(|(b1, b2)| b1.combine(&b2, '.'))
            .collect::<Vec<_>>();

        valid_xmas_boards
            .iter()
            .map(|xmas_board| input_board.find_overlaps(xmas_board, Some('.')).len() as i32)
            .sum()
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
struct Board {
    letters: HashMap<Point, char>,
    width: i32,
    height: i32,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let height = lines.len() as i32;
        let width = lines.first().unwrap_or(&"").len() as i32;
        let mut letters = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                letters.insert(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    ch,
                );
            }
        }
        Board {
            letters,
            width,
            height,
        }
    }

    /// combines two boards so that:
    /// self.letter == mask_character -> other.letter
    /// otherwise -> self.letter
    ///
    /// returns None, if cannot overlap. This will happen only if sizes are different
    fn combine(&self, other: &Board, mask_character: char) -> Option<Self> {
        if other.width != self.width || other.height != self.height {
            return None;
        }

        let mut letters = HashMap::new();

        for (point, my_letter) in &self.letters {
            let other_letter = other
                .letters
                .get(point)
                .expect("other board broke invariant");
            let ch = match (my_letter, other_letter) {
                (_, ch) if my_letter == &mask_character => ch,
                (ch, _) => ch,
            };
            letters.insert(point.clone(), *ch);
        }

        Some(Board {
            letters,
            width: self.width,
            height: self.height,
        })
    }
    /// returns new board rotated right
    /// for example will turn
    /// ```
    /// X.
    /// .M
    /// A.
    /// .S
    /// ```
    /// into
    /// ```
    /// .A.X
    /// S.M.
    /// ```
    fn rotate_right(&self) -> Self {
        let mut letters = HashMap::new();

        for (point, ch) in &self.letters {
            letters.insert(
                Point {
                    x: self.height - point.y - 1,
                    y: point.x,
                },
                *ch,
            );
        }
        Board {
            letters,
            width: self.height,
            height: self.width,
        }
    }

    fn rotate_right_many_times(&self, mut times: u32) -> Self {
        if times == 0 {
            return self.clone();
        }

        let mut ret_board = self.rotate_right();
        times -= 1;

        for _ in 0..times {
            ret_board = ret_board.rotate_right();
        }

        ret_board
    }

    /// finds overlaps of `other` Board on this Board
    /// and returns list of top-left corners of each overlap found
    /// if no overlaps are found, the list will be empty
    /// if `mask_char` is specified, then it is used to denote 'any' for comparison purposes
    fn find_overlaps(&self, other: &Board, mask_char: Option<char>) -> Vec<Point> {
        let mut result = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut found_match = true;
                for other_y in 0..other.height {
                    if !found_match {
                        break;
                    }
                    for other_x in 0..other.width {
                        let my_x = x + other_x;
                        let my_y = y + other_y;

                        let other_char = other
                            .letters
                            .get(&Point {
                                x: other_x,
                                y: other_y,
                            })
                            .unwrap_or_else(|| {
                                panic!("other board contents do not match its size {other:?}")
                            });

                        let my_char = self.letters.get(&Point { x: my_x, y: my_y });

                        match my_char {
                            Some(ch) if ch == other_char => continue,
                            Some(_) if mask_char.is_some_and(|mask| mask == *other_char) => {
                                continue
                            }
                            _ => found_match = false,
                        };
                    }
                }
                if found_match {
                    result.push(Point { x, y });
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1() {
        assert_eq!(Day4::part1(SAMPLE_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day4::part2(SAMPLE_INPUT), 9);
    }
}
