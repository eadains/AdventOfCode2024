use crate::helpers;
use regex::Regex;
use std::fs;

fn compute_mult_sums(content: &str) -> i32 {
    // TODO: Recompiling this on each function call is undesirable
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let args = re.captures_iter(&content).map(|caps| {
        // Array sizes must be known at compile time, and our regex has two capture groups
        // so every array will have size 2
        let (_, args): (_, [&str; 2]) = caps.extract();
        args.map(|x| x.parse::<i32>().expect("Error parsing to ints"))
    });
    args.map(|[x, y]| x * y).sum()
}

fn with_conditionals(content: &str) -> i32 {
    let ops_re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))").unwrap();
    let ops: Vec<&str> = ops_re
        .captures_iter(&content)
        .map(|caps| {
            let (_, [matches]) = caps.extract();
            matches
        })
        .collect();

    let mut enabled = true;
    let mut sum = 0;
    for i in 0..ops.len() {
        if ops[i].contains("mul") {
            if enabled {
                sum += compute_mult_sums(&ops[i]);
            } else if !enabled {
                continue;
            }
        } else if ops[i].contains("don't") {
            enabled = false;
        } else if ops[i].contains("do") & !enabled {
            enabled = true;
        }
    }

    sum
}

pub fn solve() {
    let content = fs::read_to_string("inputs/day3").expect("File read error");
    let ans1 = compute_mult_sums(&content);
    let ans2 = with_conditionals(&content);

    helpers::print_answers(3, ans1, ans2);
}
