use counter::Counter;

advent_of_code::solution!(1);

pub fn lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let input_lns = input.trim_end().split("\n");

    let mut list_1 = vec![];
    let mut list_2 = vec![];
    for ln in input_lns {
        let mut iter = ln.split_whitespace();
        let entry_1_txt = iter.next().expect("first entry must be present");
        let entry_1 = entry_1_txt.parse::<u32>().expect("entry must be a number");
        list_1.push(entry_1);
        let entry_2_txt = iter.next().expect("second entry must be present");
        let entry_2 = entry_2_txt.parse::<u32>().expect("entry must be a number");
        list_2.push(entry_2);
    }

    list_1.sort();
    list_2.sort();

    (list_1, list_2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (list_1, list_2) = lists(input);

    let result: u32 = list_1
        .into_iter()
        .zip(list_2)
        .map(|(entry_1, entry_2)| -> u32 {
            // convert to signed to get absolute difference before converting back to unsigned
            let entry_1: i32 = entry_1.try_into().expect("entry fits into signed i32");
            let entry_2: i32 = entry_2.try_into().expect("entry fits into signed i32");
            (entry_1 - entry_2)
                .abs()
                .try_into()
                .expect("result must be positive")
        })
        .collect::<Vec<_>>()
        .into_iter()
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list_1, list_2) = lists(input);

    let list_2_counts = list_2.iter().collect::<Counter<_>>();

    let result: u32 = list_1
        .into_iter()
        .map(|entry| -> u32 {
            let count: u32 = list_2_counts[&entry]
                .try_into()
                .expect("Number of entries in file is 10k; count is much smaller than this");
            let score = entry * count;
            score
        })
        .collect::<Vec<_>>()
        .into_iter()
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
