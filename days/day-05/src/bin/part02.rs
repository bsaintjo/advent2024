use std::io::BufRead;

use day_05::count_wrong_middle_pages;

fn main() {
    let file = advent2024::open_file_cmd_line();
    let iter = file.lines().map_while(Result::ok);
    println!("{}", count_wrong_middle_pages(iter));
}
