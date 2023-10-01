use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct SolverInput {
    problems_by_solution: HashMap<u32, HashSet<u32>>,
}

impl SolverInput {
    pub fn new(problems_by_solution: HashMap<u32, HashSet<u32>>) -> SolverInput {
        SolverInput {
            problems_by_solution,
        }
    }

    pub fn add_solution(&mut self, solution: u32, problems: HashSet<u32>) {
        self.problems_by_solution.insert(solution, problems);
    }
}

impl Default for SolverInput {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

pub struct SolverOutput {
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

pub enum Solver {
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
            for (solution, problems) in input.problems_by_solution.iter() {
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
    use std::hash::Hash;

    use super::*;

    #[test]
    fn test_basic_functionality() {
        let mut input = SolverInput::new(HashMap::new());
        input.add_solution(1, vec![1, 2, 3].into_iter().collect());
        input.add_solution(2, vec![2, 4].into_iter().collect());
        input.add_solution(3, vec![3, 5].into_iter().collect());

        let problems: HashSet<u32> = vec![1, 2, 3, 4, 5].into_iter().collect();

        let solver = Solver::GreedySolver(input);
        let result = solver.solve(problems);

        let expected_best_solutions: HashSet<u32> = vec![1, 2, 3].into_iter().collect();
        let expected_unsolved_problems: HashSet<u32> = HashSet::new();

        assert_eq!(
            *result.get_best_solutions(),
            expected_best_solutions,
            "Unexpected best solutions"
        );
        assert_eq!(
            *result.get_unsolved_problems(),
            expected_unsolved_problems,
            "Unexpected unsolved problems"
        );
    }

    #[test]
    fn test_empty_input() {
        let input = SolverInput::new(HashMap::new());
        let problems: HashSet<u32> = HashSet::new();

        let solver = Solver::GreedySolver(input);
        let result = solver.solve(problems);

        let expected_best_solutions: HashSet<u32> = HashSet::new();
        let expected_unsolved_problems: HashSet<u32> = HashSet::new();

        assert_eq!(
            *result.get_best_solutions(),
            expected_best_solutions,
            "Unexpected best solutions for empty input"
        );
        assert_eq!(
            *result.get_unsolved_problems(),
            expected_unsolved_problems,
            "Unexpected unsolved problems for empty input"
        );
    }

    #[test]
    fn test_single_set_covers_all() {
        let mut input = SolverInput::new(HashMap::new());
        input.add_solution(1, vec![1, 2, 3].into_iter().collect());

        let problems: HashSet<u32> = vec![1, 2, 3].into_iter().collect();

        let solver = Solver::GreedySolver(input);
        let result = solver.solve(problems);

        let expected_best_solutions: HashSet<u32> = vec![1].into_iter().collect();
        let expected_unsolved_problems: HashSet<u32> = HashSet::new();

        assert_eq!(
            *result.get_best_solutions(),
            expected_best_solutions,
            "Unexpected best solutions when a single set covers all"
        );
        assert_eq!(
            *result.get_unsolved_problems(),
            expected_unsolved_problems,
            "Unexpected unsolved problems when a single set covers all"
        );
    }

    #[test]
    fn test_no_solution_found() {
        let input = SolverInput::new(HashMap::new());
        let problems: HashSet<u32> = vec![1, 2, 3].into_iter().collect();

        let solver = Solver::GreedySolver(input);
        let result = solver.solve(problems);

        let expected_best_solutions: HashSet<u32> = HashSet::new();
        let expected_unsolved_problems: HashSet<u32> = vec![1, 2, 3].into_iter().collect();

        assert_eq!(
            *result.get_best_solutions(),
            expected_best_solutions,
            "Unexpected best solutions when no solution found"
        );
        assert_eq!(
            *result.get_unsolved_problems(),
            expected_unsolved_problems,
            "Unexpected unsolved problems when no solution found"
        );
    }
}
