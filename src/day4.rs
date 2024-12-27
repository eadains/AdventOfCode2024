use crate::helpers;

fn coords_to_string<const N: usize>(content: &[Vec<char>], coords: &[(isize, isize); N]) -> String {
    // Trasnform array of (x, y) coordinate pairs to a String of characters selected
    // from input matrix
    coords
        .iter()
        .map(|(x, y)| {
            // The get function returns an Option that is None if the index is out of bounds
            // automatically handling cases where there aren't enough characters in whatever
            // direction we're looking
            content
                .get(*x as usize)
                .and_then(|row| row.get(*y as usize))
                // If either index is out-of-bounds, return this default value
                .unwrap_or(&'-')
        })
        .collect()
}

pub fn solve() {
    let content: Vec<Vec<char>> = include_str!("../inputs/day4")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    // Stolen from: https://github.com/timvisee/advent-of-code-2024/blob/master/day04a/src/main.rs
    let ans1 = (0..content.len())
        // Generates tuples of every coordinate in the matrix: (row, col)
        // Must convert to isize to support negative numbers
        .flat_map(|x| (0..content[x].len()).map(move |y| (x as isize, y as isize)))
        // Generates arrays of coordinates corresponding to each direction we want to check given the starting coordinate
        .flat_map(|(x, y)| {
            [
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)], // Down
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)], // Right
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // Down-right
                [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)], // Up-right
            ]
        })
        .filter(|coords| {
            let s = coords_to_string(&content, coords);
            s == "XMAS" || s == "SAMX"
        })
        .count();

    let ans2 = (0..content.len())
        .flat_map(|x| (0..content[x].len()).map(move |y| (x as isize, y as isize)))
        .map(|(x, y)| {
            [
                [(x - 1, y - 1), (x, y), (x + 1, y + 1)], // Upper-left to lower-right
                [(x + 1, y - 1), (x, y), (x - 1, y + 1)], // Lower-left to upper-right
            ]
        })
        .filter(|[coords1, coords2]| {
            let s1 = coords_to_string(&content, coords1);
            let s2 = coords_to_string(&content, coords2);
            (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM")
        })
        .count();

    helpers::print_answers(4, ans1, ans2);
}
