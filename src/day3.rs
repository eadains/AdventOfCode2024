use crate::helpers;
use regex::Regex;
use std::{fs, sync::LazyLock};

fn compute_mult_sums(content: &str) -> i32 {
    // Only compile this once
    static MULT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
    // TODO: Recompiling this on each function call is undesirable
    let args = MULT_RE.captures_iter(content).map(|caps| {
        // Array sizes must be known at compile time, and our regex has two capture groups
        // so every array will have size 2
        let (_, args): (_, [&str; 2]) = caps.extract();
        args.map(|x| x.parse::<i32>().expect("Error parsing to ints"))
    });
    args.map(|[x, y]| x * y).sum()
}

fn with_conditionals(content: &str) -> i32 {
    // Only compile this once
    // Matches 'do()' 'don't()' and 'mul(*,*)'
    static OPS_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))").unwrap());

    let ops: Vec<&str> = OPS_RE
        .captures_iter(content)
        .map(|caps| {
            let (_, [matches]) = caps.extract();
            matches
        })
        .collect();

    // We have list of all relevant operators, keep track of whether
    // we are supposed to execute by looking at "do" and "don't" operators
    // and add up the mults as we go along
    let mut enabled = true;
    let mut sum = 0;
    for op in ops {
        if op.contains("mul") {
            if enabled {
                sum += compute_mult_sums(op);
            } else if !enabled {
                continue;
            }
        } else if op.contains("don't") {
            enabled = false;
        } else if op.contains("do") & !enabled {
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
