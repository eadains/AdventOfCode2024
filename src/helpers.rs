use std::{fmt::Display, fs};

pub fn get_line_items(path: &str) -> Vec<Vec<i32>> {
    let mut line_items: Vec<Vec<i32>> = Vec::new();
    let content = fs::read_to_string(path).expect("File read error");

    for line in content.lines() {
        line_items.push(
            line.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Failed converting to int"))
                .collect(),
        );
    }

    line_items
}

pub fn print_answers(day_num: i32, ans1: impl Display, ans2: impl Display) {
    println!("Day {day_num} Answers:");
    println!("    (part 1): {ans1}");
    println!("    (part 2): {ans2}");
}
