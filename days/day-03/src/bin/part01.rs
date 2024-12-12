use std::io::Read;

use day_03::uncorrupted;

fn main() {
    let mut reader = advent2024::open_file_cmd_line();
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    println!("{}", uncorrupted(&buf));
}