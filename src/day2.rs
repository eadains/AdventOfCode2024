use std::cmp::Ordering;

use crate::helpers;

fn check_strictly_monotone(v: &Vec<i32>) -> bool {
    let increasing = match v[1].cmp(&v[0]) {
        // If first two elements are already not strictly monotone we can exit immediately
        Ordering::Equal => return false,
        Ordering::Greater => true,
        Ordering::Less => false,
    };

    for i in 2..v.len() {
        let diff = v[i] - v[i - 1];
        if diff == 0 {
            // If not strictly monotone
            return false;
        } else if (diff < 0) & increasing {
            // If should be increasing but isn't
            return false;
        } else if (diff > 0) & !increasing {
            // If should be decreasing but isn't
            return false;
        }
    }
    return true;
}

fn check_differences(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        let diff = (v[i] - v[i - 1]).abs();
        if diff < 1 {
            return false;
        } else if diff > 3 {
            return false;
        }
    }
    return true;
}

fn remove_one_check(v: &Vec<i32>) -> bool {
    for i in 0..v.len() {
        let dropped_v = [&v[..i], &v[i + 1..]].concat();
        if check_strictly_monotone(&dropped_v) & check_differences(&dropped_v) {
            return true;
        }
    }
    return false;
}

pub fn solve() {
    let reports = helpers::get_line_items("inputs/day2");
    let ans1 = reports
        .iter()
        .filter(|&x| check_strictly_monotone(x) & check_differences(x))
        .count();

    let ans2 = reports.iter().filter(|&x| remove_one_check(x)).count();

    helpers::print_answers(2, ans1, ans2);
}
