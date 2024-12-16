use std::iter::repeat;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct WordSearch {
    row: usize,
    col: usize,
}

impl WordSearch {
    fn calc_vectors(&self, puzzle: &Puzzle) -> Vec<Vec<(usize, usize)>> {
        let mut acc = Vec::new();
        let left_ok = self.col.checked_sub(3).is_some();
        let up_ok = self.row.checked_sub(3).is_some();
        let right_ok = (self.col + 4) < puzzle.n_cols();
        let down_ok = (self.row + 4) < puzzle.n_rows();

        if left_ok {
            acc.push(repeat(self.row).zip(self.col - 3..=self.col).collect_vec());
        }

        if left_ok && down_ok {
            acc.push(
                (self.row..self.row + 4)
                    .rev()
                    .zip(self.col - 3..=self.col)
                    .collect_vec(),
            );
        }

        if down_ok {
            acc.push((self.row..self.row + 4).zip(repeat(self.col)).collect_vec());
        }

        if right_ok && down_ok {
            acc.push(
                (self.row..self.row + 4)
                    .zip(self.col..self.col + 4)
                    .collect_vec(),
            );
        }

        if right_ok {
            acc.push(repeat(self.row).zip(self.col..self.col + 4).collect_vec());
        }

        if right_ok && up_ok {
            acc.push(
                (self.row - 3..=self.row)
                    .rev()
                    .zip(self.col..self.col + 4)
                    .collect_vec(),
            );
        }
        if up_ok {
            acc.push(
                (self.row - 3..=self.row)
                    .zip(repeat(self.col))
                    .collect_vec(),
            );
        }
        if left_ok && up_ok {
            acc.push(
                (self.row - 3..=self.row)
                    .zip(self.col - 3..=self.col)
                    .collect_vec(),
            );
        }
        acc
    }
}

pub struct Puzzle {
    letters: Vec<Vec<u8>>,
}

impl<B> FromIterator<B> for Puzzle
where
    B: Into<Vec<u8>>,
{
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        Puzzle {
            letters: iter.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl Puzzle {
    fn n_cols(&self) -> usize {
        self.letters[0].len()
    }

    fn n_rows(&self) -> usize {
        self.letters.len()
    }

    fn get_starts(&self) -> Vec<WordSearch> {
        let mut acc = Vec::new();
        for (row_idx, row) in self.letters.iter().enumerate() {
            for (col_idx, &col) in row.iter().enumerate() {
                if col == b'X' {
                    acc.push(WordSearch {
                        row: row_idx,
                        col: col_idx,
                    })
                }
            }
        }
        acc
    }
    fn spell(&self, idxs: &[(usize, usize)]) -> Vec<u8> {
        idxs.iter()
            .map(|&(row, col)| self.letters[row][col])
            .collect()
    }

    pub fn count_xmas(&self) -> usize {
        let mut count = 0;
        let wss = self.get_starts();
        for ws in wss {
            let valid_idxs = ws.calc_vectors(self);
            let mut local_count = 0;
            for valid_idx in valid_idxs {
                let res = String::from_utf8(self.spell(&valid_idx)).unwrap();
                if (res == "XMAS") || (res == "SAMX") {
                    local_count += 1;
                }
            }
            // if local_count > 0 {
            //     println!("{ws:?}, {local_count}");
            // }
            count += local_count;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part01() {
        let letters = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let puzzle = Puzzle::from_iter(letters.split_whitespace());
        let ws = WordSearch { row: 5, col: 9 };

        assert_eq!(
            String::from_utf8(puzzle.spell(&[(1, 4), (1, 3), (1, 2), (1, 1)])).unwrap(),
            "XMAS"
        );
        let mut count = 0;
        let wss = puzzle.get_starts();
        for ws in wss {
            let valid_idxs = ws.calc_vectors(&puzzle);
            let mut local_count = 0;
            for valid_idx in valid_idxs {
                let res = String::from_utf8(puzzle.spell(&valid_idx)).unwrap();
                if (res == "XMAS") || (res == "SAMX") {
                    local_count += 1;
                }
            }
            if local_count > 0 {
                println!("{ws:?}, {local_count}");
            }
            count += local_count;
        }
        println!("{count}");
    }
}
