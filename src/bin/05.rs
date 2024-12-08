use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

fn solution(input: &str) -> (u32, u32) {
    let mut input_iter = input.split("\n\n");
    let rules_input = input_iter.next().expect("Rules will be present");
    let updates_input = input_iter.next().expect("Updates should be present");
    assert_eq!(None, input_iter.next());

    let mut rules = HashMap::new();
    for rule in rules_input.lines() {
        let mut rule_iter = rule.split("|");
        let first_num: u32 = rule_iter
            .next()
            .expect("First number present")
            .parse()
            .expect("Fits into u32");
        let second_num: u32 = rule_iter
            .next()
            .expect("Second number present")
            .parse()
            .expect("Fits into u32");
        rules.insert((first_num, second_num), first_num);
        rules.insert((second_num, first_num), first_num);
    }

    let updates: Vec<Vec<u32>> = updates_input
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|page| page.parse().expect("Fits into u32"))
                .collect()
        })
        .collect();

    let mut unordered_updates = vec![];
    let mut result = 0;
    'outer: for update in updates {
        for combo in update.iter().combinations(2) {
            let first_num = rules.get(&(*combo[0], *combo[1])).expect("");
            if *combo[0] != *first_num {
                unordered_updates.push(update);
                continue 'outer;
            }
        }
        result += update[update.len() / 2];
    }

    let mut result_2 = 0;
    for update in unordered_updates.into_iter() {
        let mut combos = HashMap::new();
        for combo in update.iter().combinations(2) {
            let page_1 = combos.entry(combo[0]).or_insert(HashSet::new());
            page_1.insert(combo.clone());
            let page_2 = combos.entry(combo[1]).or_insert(HashSet::new());
            page_2.insert(combo.clone());
        }

        let mut new_update = vec![];
        while new_update.len() != update.len() {
            let mut found_earliest = false;
            let mut earliest_page = None;
            for (page, page_combos) in combos.clone().iter() {
                if !(combos.len() == 1
                    || page_combos
                        .iter()
                        .all(|page_combo| **page == rules[&(*page_combo[0], *page_combo[1])]))
                {
                    continue;
                }
                found_earliest = true;
                earliest_page = Some(**page);
                new_update.push(**page);
                combos.remove(page);
                break;
            }
            let earliest_page = earliest_page.expect(
                "Combos must be populated whilst new_update is not the same length as update",
            );
            if found_earliest {
                for page_combos in combos.values_mut() {
                    for page_combo in page_combos.clone() {
                        if page_combo.contains(&&earliest_page) {
                            page_combos.remove(&page_combo);
                        }
                    }
                }
                continue;
            }
        }
        result_2 += new_update[new_update.len() / 2];
    }
    (result, result_2)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solution(input).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solution(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_one_with_multiple_updates_with_same_midpoint() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(286));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(123));
    }
}
