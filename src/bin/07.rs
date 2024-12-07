advent_of_code::solution!(7);

struct Equation {
    result: u64,
    operands: Vec<u64>,
}

struct EquationSolver {
    concat: bool,
}

impl EquationSolver {
    fn new(concat: bool) -> Self {
        EquationSolver { concat }
    }

    fn solve_eq(&self, mut operands: Vec<u64>, result: u64) -> bool {
        let operand = operands.pop();
        let operand = match operand {
            None => return result == 0,
            Some(operand) => operand,
        };

        // try concat
        if self.concat {
            let op_str = operand.to_string();
            let result_str = result.to_string();
            if result_str.ends_with(&op_str) {
                let (concat_result_str, _) = result_str.split_at(result_str.len() - op_str.len());
                let concat_result = if concat_result_str == "" {
                    0
                } else {
                    concat_result_str
                        .parse()
                        .expect("u64 split into two will fit into u64")
                };
                let solved = self.solve_eq(operands.clone(), concat_result);
                if solved {
                    return solved;
                }
            }
        }

        // try div
        if result % operand == 0 {
            let div_result = result / operand;
            let solved = self.solve_eq(operands.clone(), div_result);
            if solved {
                return solved;
            }
        }

        // div did not work - attempt sub
        let sub_result = result.checked_sub(operand);
        match sub_result {
            None => false,
            Some(sub_result) => self.solve_eq(operands, sub_result),
        }
    }

    fn solve_eqs(&self, input: &str) -> u64 {
        let mut eqs = vec![];
        for eq_str in input.lines() {
            let mut eq_iter = eq_str.split(": ");
            let result_str = eq_iter.next().expect("result should be present");
            let operands_str = eq_iter.next().expect("operands should be present");
            let operands: Vec<u64> = operands_str
                .split_whitespace()
                .map(|op| op.parse().expect("All should be u32"))
                .collect();
            eqs.push(Equation {
                result: result_str.parse().expect("Should be u32"),
                operands,
            })
        }

        let mut result = 0;
        for eq in eqs {
            let eq_result = eq.result;
            if self.solve_eq(eq.operands, eq.result) {
                result += eq_result
            }
        }

        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let eq_solver = EquationSolver::new(false);
    Some(eq_solver.solve_eqs(input))
}

pub fn part_two(input: &str) -> Option<u64> {
    let eq_solver = EquationSolver::new(true);
    Some(eq_solver.solve_eqs(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
