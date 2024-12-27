use crate::helpers;

fn read_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let line_items = helpers::get_line_items(path);
    for items in line_items {
        list1.push(items[0]);
        list2.push(items[1]);
    }

    (list1, list2)
}

fn compute_total_distance(list1: &[i32], list2: &[i32]) -> i32 {
    let it = list1.iter().zip(list2.iter());
    it.map(|(x, y)| (x - y).abs()).sum()
}

fn count_occurrences(value: &i32, v: &[i32]) -> i32 {
    v.iter().filter(|&x| x == value).count().try_into().unwrap()
}

pub fn solve() {
    let (mut list1, mut list2) = read_file("inputs/day1");
    list1.sort();
    list2.sort();
    let ans1 = compute_total_distance(&list1, &list2);
    let ans2: i32 = list1.iter().map(|x| x * count_occurrences(x, &list2)).sum();

    helpers::print_answers(1, ans1, ans2);
}
