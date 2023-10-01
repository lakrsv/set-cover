use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct SolverInput {
    problems_by_solution: HashMap<u32, HashSet<u32>>,
}

impl SolverInput {
    fn new() -> SolverInput {
        SolverInput {
            problems_by_solution: HashMap::new(),
        }
    }
    fn add_solution(&mut self, solution: u32, problems: HashSet<u32>) {
        self.problems_by_solution.insert(solution, problems);
    }
}

struct SolverOutput {
    best_solutions: HashSet<u32>,
    unsolved_problems: HashSet<u32>,
}

impl SolverOutput {
    pub fn get_best_solutions(&self) -> &HashSet<u32> {
        &self.best_solutions
    }
    pub fn get_unsolved_problems(&self) -> &HashSet<u32> {
        &self.unsolved_problems
    }
}

enum Solver {
    GreedySolver(SolverInput),
}

impl Solver {
    pub fn solve(&self, problems: HashSet<u32>) -> SolverOutput {
        match self {
            Solver::GreedySolver(input) => self.solve_greedy(problems, input),
        }
    }

    fn solve_greedy(&self, problems: HashSet<u32>, input: &SolverInput) -> SolverOutput {
        let mut universe: HashSet<u32> = problems;
        let mut best_solutions: HashSet<u32> = HashSet::new();
        while !universe.is_empty() {
            let mut best_solution: u32 = 0;
            let mut best_intersection: HashSet<_> = HashSet::new();
            for (solution, problems) in &input.problems_by_solution {
                let intersection: HashSet<_> = universe.intersection(problems).copied().collect();
                if intersection.len() > best_intersection.len() {
                    best_solution = *solution;
                    best_intersection = intersection;
                }
            }

            if best_intersection.is_empty() {
                return SolverOutput {
                    best_solutions,
                    unsolved_problems: universe,
                };
            }

            best_solutions.insert(best_solution);
            for result in best_intersection {
                universe.remove(&result);
            }
        }

        SolverOutput {
            best_solutions,
            unsolved_problems: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_solve_1() {
        let mut input = SolverInput::new();
        input.add_solution(0, HashSet::from([10]));
        input.add_solution(1, HashSet::from([30]));
        input.add_solution(2, HashSet::from([10, 20, 30, 40]));

        let output = Solver::GreedySolver(input).solve(HashSet::from([10, 20, 30, 40]));

        assert_eq!(HashSet::from([2]), output.get_best_solutions().clone());
    }
    #[test]
    fn test_greedy_solve_2() {
        let mut input = SolverInput::new();
        input.add_solution(0, HashSet::from([10, 20, 30]));
        input.add_solution(1, HashSet::from([10, 20]));
        input.add_solution(2, HashSet::from([50, 60, 70]));
        input.add_solution(3, HashSet::from([70, 80, 90]));
        input.add_solution(4, HashSet::from([100]));
        input.add_solution(5, HashSet::from([110, 120, 130]));
        input.add_solution(6, HashSet::from([130, 140, 150]));

        let output = Solver::GreedySolver(input)
            .solve(HashSet::from([10, 20, 30, 60, 80, 100, 120, 130, 140]));

        assert_eq!(
            HashSet::from([0, 2, 3, 4, 5, 6]),
            output.get_best_solutions().clone()
        );
    }

    #[test]
    fn test_greedy_solve_3() {
        let mut input = SolverInput::new();
        input.add_solution(0, HashSet::from([10]));
        input.add_solution(1, HashSet::from([20]));
        input.add_solution(2, HashSet::from([30]));

        let output = Solver::GreedySolver(input)
            .solve(HashSet::from([10, 20, 30]));

        assert_eq!(
            HashSet::from([0, 1, 2]),
            output.get_best_solutions().clone()
        );
    }

    #[test]
    fn test_greedy_solve_4() {
        let mut input = SolverInput::new();
        input.add_solution(0, HashSet::from([10]));
        input.add_solution(1, HashSet::from([20, 30]));
        input.add_solution(2, HashSet::from([30]));

        let output = Solver::GreedySolver(input)
            .solve(HashSet::from([10, 20, 30]));

        assert_eq!(
            HashSet::from([0, 1]),
            output.get_best_solutions().clone()
        );
    }


    #[test]
    fn test_greedy_solve_no_solution() {
        let mut input = SolverInput::new();
        input.add_solution(0, HashSet::from([10, 20, 30]));

        let output = Solver::GreedySolver(input).solve(HashSet::from([40]));

        assert_eq!(HashSet::new(), output.get_best_solutions().clone());
        assert_eq!(HashSet::from([40]), output.get_unsolved_problems().clone());
    }
}
