use regex::Regex;

advent_of_code::solution!(3);

fn scan(input: &str, conditional: bool) -> u32 {
    let re = Regex::new(
        r"(?:mul\((?<mul_x>\d{1,3}),(?<mul_y>\d{1,3})\))|(?<conditional>do\(\)|don't\(\))",
    )
    .expect("Pattern should be valid");
    let instrs = re.captures_iter(input);

    let mut enabled_instrs = vec![];
    let mut do_ = true;
    for instr_capture in instrs {
        let conditional_match = instr_capture.name("conditional").map(|m| m.as_str());
        if conditional_match == Some("don't()") {
            if conditional {
                do_ = false;
            }
        } else if conditional_match == Some("do()") {
            do_ = true;
        } else if do_ {
            let mul_x: u32 = instr_capture["mul_x"]
                .parse()
                .expect("Should be u32 integer");
            let mul_y: u32 = instr_capture["mul_y"]
                .parse()
                .expect("Should be u32 integer");
            enabled_instrs.push(mul_x * mul_y);
        }
    }
    enabled_instrs.iter().sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(scan(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(scan(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
