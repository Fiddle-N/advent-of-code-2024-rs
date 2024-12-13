use num_rational::Rational32;
use regex::Regex;

advent_of_code::solution!(13);

const MACHINE_PATTERN: &str = concat!(
    r"Button A: X\+(?P<a_x>\d+), Y\+(?P<a_y>\d+)\n",
    r"Button B: X\+(?P<b_x>\d+), Y\+(?P<b_y>\d+)\n",
    r"Prize: X=(?P<prize_x>\d+), Y=(?P<prize_y>\d+)",
);

const PRIZE_TRANSLATION: i64 = 10000000000000;

struct Point {
    x: i64,
    y: i64,
}

struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn parse(input: &str, prize_translation: i64) -> Vec<Machine> {
    input
        .trim_end()
        .split("\n\n")
        .map(|machine_data| {
            let match_ = Regex::new(MACHINE_PATTERN)
                .expect("Pattern should be valid")
                .captures(machine_data)
                .expect("Should match");
            Machine {
                a: Point {
                    x: match_["a_x"].parse().expect("Should be i64 integer"),
                    y: match_["a_y"].parse().expect("Should be i64 integer"),
                },
                b: Point {
                    x: match_["b_x"].parse().expect("Should be i64 integer"),
                    y: match_["b_y"].parse().expect("Should be i64 integer"),
                },
                prize: Point {
                    x: match_["prize_x"]
                        .parse::<i64>()
                        .expect("Should be i64 integer")
                        + prize_translation,
                    y: match_["prize_y"]
                        .parse::<i64>()
                        .expect("Should be i64 integer")
                        + prize_translation,
                },
            }
        })
        .collect()
}

/// Calculate the cost of pressing buttons to win the prize for each machine
///
/// Given the following machine:
/// Button A: X+94, Y+34
/// Button B: X+22, Y+67
/// Prize: X=8400, Y=5400
///
/// This can be written using the following equation syntax:
/// 94a + 22b = 8400
/// 34a + 67b = 5400
/// which is just a system of linear equations.
///
/// Firstly we can check the gradients of the two lines are not parallel.
/// If we rewrite both equations in the form y = mx + c:
/// a = (8400 - 22b) / 94
/// a = 8400/94 - 22/94 * b
///
/// a = (5400 - 67b) / 34
/// a = 5400/34 - 67/34 * b
///
/// 22/94 != 67/34; hence the lines are not parallel.
///
/// We can then solve the system of equations using the substitution method.
/// First, we can rearrange the first equation to solve for b:
/// 22b = 8400 - 94a
/// b = (8400 - 94a) / 22
///
/// We can then substitute this value of b into the second equation:
/// 34a + 67(8400/22 - 94/22 * a) = 5400
/// 34a + 67(8400/22) - 67(94/22) * a = 5400
/// 34a - 67(94/22) * a = 5400 - 67(8400/22)
/// 22 * 34a - 67 * 94a = 22 * 5400 - 67 * 8400
/// a = (22 * 5400 - 67 * 8400) / (22 * 34 - 67 * 94)
///
/// This means that a = (b_x * prize_y - b_y * prize_x) / (b_x * a_y - a_x * b_y)
///
/// We can then substitute this value of a back into the first equation to solve for b:
/// b = (8400 - 94a) / 22
///
/// This means that b = (prize_x - a_x * a) / b_x
///
/// Once we have the values of a and b, we can calculate the cost of pressing the buttons
/// which is just 3 * a + b
fn calc_costs(machines: Vec<Machine>) -> u64 {
    machines
        .into_iter()
        .filter_map(|machine| {
            let machine_a_x_i32: i32 = machine.a.x.try_into().expect("Fits into i32");
            let machine_b_x_i32: i32 = machine.b.x.try_into().expect("Fits into i32");
            let machine_a_y_i32: i32 = machine.a.y.try_into().expect("Fits into i32");
            let machine_b_y_i32: i32 = machine.b.y.try_into().expect("Fits into i32");
            let m_x = Rational32::new(-machine_a_x_i32, machine_b_x_i32);
            let m_y = Rational32::new(-machine_a_y_i32, machine_b_y_i32);
            if m_x == m_y {
                panic!("Lines are parallel");
            }

            let a_numer = machine.b.x * machine.prize.y - machine.b.y * machine.prize.x;
            let a_denom = machine.b.x * machine.a.y - machine.a.x * machine.b.y;
            if a_numer % a_denom != 0 {
                // No solution for this machine
                return None;
            }
            let a = a_numer / a_denom;

            let b_numer = machine.prize.x - machine.a.x * a;
            let b_denom = machine.b.x;
            if b_numer % b_denom != 0 {
                // No solution for this machine
                return None;
            }
            let b = b_numer / b_denom;

            let a: u64 = a.try_into().expect("Fits into u64");
            let b: u64 = b.try_into().expect("Fits into u64");

            Some(3 * a + b)
        })
        .sum()
}

fn solve(input: &str, prize_translation: i64) -> u64 {
    let machines = parse(input, prize_translation);
    calc_costs(machines)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 0))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, PRIZE_TRANSLATION))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
