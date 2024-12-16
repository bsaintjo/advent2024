use std::iter::repeat;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct WordSearch {
    row: usize,
    col: usize,
}

type CrossPosition = (Vec<(usize, usize)>, Vec<(usize, usize)>);

impl WordSearch {
    fn calc_vectors_cross(&self, puzzle: &Puzzle) -> Option<CrossPosition> {
        let left_ok = self.col > 0;
        let up_ok = self.row > 0;
        let right_ok = (self.col + 1) < puzzle.n_cols();
        let down_ok = (self.row + 1) < puzzle.n_rows();

        if left_ok && up_ok && right_ok && down_ok {
            let fst = vec![(self.row - 1, self.col - 1), (self.row, self.col), (self.row + 1, self.col + 1)];
            let snd = vec![(self.row - 1, self.col + 1), (self.row, self.col), (self.row + 1, self.col - 1)];
            Some((fst, snd))
        } else {
            None
        }

    }

    fn calc_vectors_eight_way(&self, puzzle: &Puzzle) -> Vec<Vec<(usize, usize)>> {
        let mut acc = Vec::new();
        let left_ok = self.col.checked_sub(3).is_some();
        let up_ok = self.row.checked_sub(3).is_some();
        let right_ok = (self.col + 3) < puzzle.n_cols();
        let down_ok = (self.row + 3) < puzzle.n_rows();

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

    fn get_starts(&self, start: u8) -> Vec<WordSearch> {
        let mut acc = Vec::new();
        for (row_idx, row) in self.letters.iter().enumerate() {
            for (col_idx, &col) in row.iter().enumerate() {
                if col == start {
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

    pub fn count_x_mas(&self) -> usize {
        let mut count = 0;
        let wss = self.get_starts(b'A');
        for ws in wss {
            if let Some((fst, snd)) = ws.calc_vectors_cross(self) {
                let fst = self.spell(&fst);
                let snd = self.spell(&snd);

                let fst_valid = (fst == b"MAS") || (fst == b"SAM");
                let snd_valid = (snd == b"MAS") || (snd == b"SAM");
                if fst_valid && snd_valid {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn count_xmas(&self) -> usize {
        let mut count = 0;
        let wss = self.get_starts(b'X');
        for ws in wss {
            let valid_idxs = ws.calc_vectors_eight_way(self);
            let mut local_count = 0;
            for valid_idx in valid_idxs {
                let res = String::from_utf8(self.spell(&valid_idx)).unwrap();
                if (res == "XMAS") || (res == "SAMX") {
                    local_count += 1;
                }
            }
            count += local_count;
        }
        count
    }
}

struct Windows<'a> {
    row: usize,
    col: usize,
    length: usize,
    width: usize,
    puzzle: &'a Puzzle
}

impl<'a> Iterator for Windows<'a> {
    type Item = &'a [&'a [u8]];

    fn next(&mut self) -> Option<Self::Item> {
        if (self.col + self.width) >= self.puzzle.n_cols() && ((self.row + self.length) >= self.puzzle.n_rows()){
            return None;
        }
        if (self.col + self.width) >=self.puzzle.n_cols() {
            self.col = 0;
            self.row += 1;
        }
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_x_mas() {
        let letters = "M.M
.A.
S.S";
        let puzzle = Puzzle::from_iter(letters.split_whitespace());
        assert_eq!(puzzle.count_x_mas(), 1);
    }

    #[test]
    fn test_top_left_corner() {
        let letters = "XMAS
MM..
A.A.
S..S";
        let puzzle = Puzzle::from_iter(letters.split_whitespace());
        assert_eq!(puzzle.count_xmas(), 3);
    }

    #[test]
    fn test_bottom_left_corner() {
        let letters = "S..S
A.A.
MM..
XMAS";
        let puzzle = Puzzle::from_iter(letters.split_whitespace());
        assert_eq!(puzzle.count_xmas(), 3);
    }

    #[test]
    fn test_top_right_corner() {
        let letters = "SAMX
..MM
.A.A
S..S";
        let puzzle = Puzzle::from_iter(letters.split_whitespace());
        assert_eq!(puzzle.count_xmas(), 3);
    }

    #[test]
    fn test_bottom_right_corner() {
        let letters = "S..S
.A.A
..MM
SAMX";
        let puzzle = Puzzle::from_iter(letters.split_whitespace());
        assert_eq!(puzzle.count_xmas(), 3);
    }


    #[test]
    fn test_part0x() {
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
        assert_eq!(puzzle.count_xmas(), 18);
        assert_eq!(puzzle.count_x_mas(), 9);
    }
}
