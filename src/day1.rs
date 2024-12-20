use std::{fs, i32};

fn read_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let content = fs::read_to_string(path).expect("File read error");
    for line in content.lines() {
        let items: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Failed converting to int"))
            .collect();
        list1.push(items[0]);
        list2.push(items[1]);
    }

    (list1, list2)
}

fn compute_total_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let it = list1.iter().zip(list2.iter());
    it.map(|(x, y)| (x - y).abs()).sum()
}

fn count_occurrences(value: &i32, v: &Vec<i32>) -> i32 {
    v.iter().filter(|&x| x == value).count().try_into().unwrap()
}

pub fn solve() {
    println!("Day 1 Answers:");
    let (mut list1, mut list2) = read_file("inputs/day1");
    list1.sort();
    list2.sort();
    println!("    (part 1): {}", compute_total_distance(&list1, &list2));

    let sim_score: i32 = list1.iter().map(|x| x * count_occurrences(x, &list2)).sum();
    println!("    (part 2): {}", sim_score);
}
