use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<u64, u64> {
    let mut stones = HashMap::new();
    for stone in input
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
    {
        let count = stones.entry(stone).or_insert(0);
        *count += 1;
    }
    stones
}

fn solve_stone(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    }
    let num_str = num.to_string();
    if num_str.len() % 2 == 0 {
        let half = num_str.len() / 2;
        return vec![
            num_str[..half].parse().unwrap(),
            num_str[half..].parse().unwrap(),
        ];
    }
    vec![num * 2024]
}

fn solve_stones(mut nums: HashMap<u64, u64>, n: u64) -> u64 {
    let mut new_nums = HashMap::new();
    for _ in 0..n {
        for (num, count) in nums.into_iter() {
            for new_num in solve_stone(num.try_into().expect("Fits into u64")) {
                let new_count = new_nums.entry(new_num).or_insert(0);
                *new_count += count;
            }
        }
        nums = new_nums;
        new_nums = HashMap::new();
    }
    nums.values().sum()
}

fn solve(input: &str, n: u64) -> u64 {
    let stones = parse(input);
    solve_stones(stones, n)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
