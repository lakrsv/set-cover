use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use set_cover::solver::{Solver, SolverInput};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    problem: Vec<u32>,
    #[arg(short, long)]
    solutions: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Solutions {
    problems_by_solution: HashMap<u32, HashSet<u32>>,
}

fn main() {
    let args = Args::parse();
    let solutions: Solutions = serde_json::from_str(
        &fs::read_to_string(args.solutions).expect("Unable to read solutions file"),
    )
    .expect("Unable to parse solutions file");

    let input = SolverInput::new(solutions.problems_by_solution);
    let solution = Solver::GreedySolver(input).solve(args.problem.iter().cloned().collect());

    print!("Best solutions: {:?}\n", solution.get_best_solutions());
    print!("Unsolved problems: {:?}\n", solution.get_unsolved_problems());
}
