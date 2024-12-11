use std::io::BufRead;

fn main() {
    let reader = advent2024::open_file_cmd_line();
    let (mut left, mut right) = day_01::parse_nums(reader.lines().map(|x| x.unwrap()));
    left.sort();
    right.sort();
    println!("{}", day_01::distance(&left, &right));
}
