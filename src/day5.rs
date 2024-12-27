use crate::helpers;
use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    let content = include_str!("../inputs/day5");

    let page_rule_re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    // We flip the rules so we can process the vector forward instead of backward.
    // Idea: We can look up each element of the line to see which elements must appear before it and then ensure
    // that the remaining elements from the line aren't any of those
    for (_, [lhs, rhs]) in page_rule_re.captures_iter(content).map(|c| c.extract()) {
        match rules.get_mut(rhs) {
            Some(x) => x.push(lhs),
            None => {
                rules.insert(rhs, vec![lhs]);
            }
        }
    }

    let lines_re = Regex::new(r"\d+(?:,\d+)+").unwrap();
    let lines = lines_re
        .find_iter(content)
        .map(|m| m.as_str().split(",").collect::<Vec<_>>());

    let ans1: i32 = lines
        .filter(|line| {
            line.iter().enumerate().all(|(i, &el)| match rules.get(el) {
                // If rule exists then rest of line must NOT contain
                // any of the values from that rule
                Some(x) => !x.iter().any(|x| line[i + 1..].contains(x)),
                // If no rule exists then it trivially passes
                None => true,
            })
        })
        // Have to subtract one from this because rust is zero-indexed
        .map(|line| line[line.len().div_ceil(2) - 1].parse::<i32>().unwrap())
        .sum();

    helpers::print_answers(5, ans1, "");
}
