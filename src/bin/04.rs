use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.lines().collect();

    fn get_lines(rows: &Vec<&str>, func: fn(i32, i32) -> i32) -> Vec<String> {
        let mut line_map = HashMap::new();
        for (y, row) in rows.iter().enumerate() {
            for (x, ele) in row.chars().enumerate() {
                let x = x.try_into().expect("Should fit into i32");
                let y = y.try_into().expect("Should fit into i32");
                let line = line_map.entry(func(x, y)).or_insert(String::new());
                line.push(ele);
            }
        }
        // return lines in any arbritrary order
        line_map.into_values().collect()
    }

    let all_lines = vec![
        get_lines(&rows, |_x, y| y),    // rows
        get_lines(&rows, |x, _y| x),    // cols
        get_lines(&rows, |x, y| x + y), // forward diagonals
        get_lines(&rows, |x, y| x - y), // backward diagonals
    ];

    fn find_xmas(line: &str) -> usize {
        line.chars()
            .tuple_windows::<(_, _, _, _)>()
            .filter(|slice| *slice == ('X', 'M', 'A', 'S'))
            .count()
    }

    let mut xmas_count = 0;
    for line_type in all_lines {
        for line in line_type {
            if line.len() < 4 {
                continue;
            }
            xmas_count += find_xmas(&line);
            xmas_count += find_xmas(&line.chars().rev().collect::<String>());
        }
    }

    Some(xmas_count.try_into().expect("Answer will fit into u32"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut xmas_count = 0;
    for (y, row) in rows.iter().enumerate() {
        if y == 0 || y == (rows.len() - 1) {
            continue;
        }
        for (x, ele) in row.iter().enumerate() {
            if x == 0 || x == (row.len() - 1) {
                continue;
            }
            if *ele != 'A' {
                continue;
            }
            let fdiag = (rows[y + 1][x - 1], *ele, rows[y - 1][x + 1]);
            if fdiag != ('M', 'A', 'S') && fdiag != ('S', 'A', 'M') {
                continue;
            }
            let bdiag = (rows[y - 1][x - 1], *ele, rows[y + 1][x + 1]);
            if bdiag != ('M', 'A', 'S') && bdiag != ('S', 'A', 'M') {
                continue;
            }
            xmas_count += 1;
        }
    }

    Some(xmas_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
