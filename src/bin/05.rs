use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

struct ManualSolver {
    rules: HashMap<(u32, u32), u32>,
    updates: Vec<Vec<u32>>,
}

struct ManualResults {
    ordered_page_sum: u32,
    unordered_page_sum: u32,
}

impl ManualSolver {
    fn new(input: &str) -> Self {
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

        Self { rules, updates }
    }

    fn get_page_pairs(&self, update: &Vec<u32>) -> HashMap<u32, HashSet<Vec<u32>>> {
        let mut page_pairs = HashMap::new();
        for page_pair in update.iter().combinations(2) {
            let page_1 = page_pairs.entry(*page_pair[0]).or_insert_with(HashSet::new);
            page_1.insert(page_pair.iter().map(|&&x| x).collect());
            let page_2 = page_pairs.entry(*page_pair[1]).or_insert_with(HashSet::new);
            page_2.insert(page_pair.iter().map(|&&x| x).collect());
        }
        page_pairs
    }

    fn remove_page_from_page_pairs(
        &self,
        page: u32,
        page_pairs: &mut HashMap<u32, HashSet<Vec<u32>>>,
    ) {
        for page_combos in page_pairs.values_mut() {
            for page_combo in page_combos.clone() {
                if page_combo.contains(&page) {
                    page_combos.remove(&page_combo);
                }
            }
        }
    }

    fn reorder_update(
        &self,
        update: &Vec<u32>,
        page_pairs: &mut HashMap<u32, HashSet<Vec<u32>>>,
    ) -> Vec<u32> {
        let mut sorted_update = vec![];
        while sorted_update.len() != update.len() {
            let mut found_earliest = false;
            let mut earliest_page = None;
            for (page, pairs) in page_pairs.clone().iter() {
                if !(page_pairs.len() == 1
                    || pairs
                        .iter()
                        .all(|page_combo| *page == self.rules[&(page_combo[0], page_combo[1])]))
                {
                    continue;
                }
                found_earliest = true;
                earliest_page = Some(*page);
                sorted_update.push(*page);
                page_pairs.remove(page);
                break;
            }
            let earliest_page = earliest_page.expect(
                "Page pairs must be populated whilst sorted_update is not the same length as update",
            );
            if found_earliest {
                self.remove_page_from_page_pairs(earliest_page, page_pairs);
                continue;
            }
        }
        sorted_update
    }

    fn solve_unordered(&self, unordered_updates: Vec<&Vec<u32>>) -> u32 {
        let mut unordered_page_sum = 0;
        for update in unordered_updates.into_iter() {
            let mut page_pairs = self.get_page_pairs(update);
            let sorted_update = self.reorder_update(update, &mut page_pairs);
            unordered_page_sum += sorted_update[sorted_update.len() / 2];
        }
        unordered_page_sum
    }

    fn solve(&self) -> ManualResults {
        let mut unordered_updates = vec![];
        let mut ordered_page_sum = 0;
        'outer: for update in self.updates.iter() {
            for page_pair in update.iter().combinations(2) {
                let left_page = self
                    .rules
                    .get(&(*page_pair[0], *page_pair[1]))
                    .expect("Page pair will be present in rules");
                if *page_pair[0] != *left_page {
                    unordered_updates.push(update);
                    continue 'outer;
                }
            }
            ordered_page_sum += update[update.len() / 2];
        }

        let unordered_page_sum = self.solve_unordered(unordered_updates);

        ManualResults {
            ordered_page_sum,
            unordered_page_sum,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(ManualSolver::new(input).solve().ordered_page_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(ManualSolver::new(input).solve().unordered_page_sum)
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
