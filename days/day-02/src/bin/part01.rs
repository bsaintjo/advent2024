use std::io::BufRead;

fn main() {
    let reader = advent2024::open_file_cmd_line();
    let mut safe_reports = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let levels = day_02::parse_level(&line);
        if day_02::is_safe(&levels) {
            safe_reports += 1;
        }
    }
    println!("{safe_reports}");
}
