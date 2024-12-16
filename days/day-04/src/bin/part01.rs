use std::io::Read;

use advent2024::open_file_cmd_line;

fn main() {
    let mut file = open_file_cmd_line();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let puzzle = day_04::Puzzle::from_iter(buf.split_whitespace());
    println!("{}", puzzle.count_xmas());
}