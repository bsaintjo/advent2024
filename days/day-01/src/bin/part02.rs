use std::io::BufRead;

fn main() {
    let reader = advent2024::open_file_cmd_line();
    let (left, right) = day_01::parse_nums(reader.lines().map(|x| x.unwrap()));
    println!("{}", day_01::similarity_score(&left, &right));
}
