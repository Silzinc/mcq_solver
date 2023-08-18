mod sheet;
mod mcq;
mod annealing;
mod parameters;
// mod io;

use mcq::{Sheet, MCQ};
use annealing::AnnealingSolver;
use parameters::*;

fn main() {
    let mut time = 0f64;
    let mut iter = 0u32;
    let mut _successes = 0u32;

    while iter < 1u32 {
        let mcq = MCQ::generate_random();
        let sheets: [Sheet; SIMULATION_SHEETS] = core::array::from_fn(|_| mcq.generate_random_sheet());

        let now = std::time::Instant::now();
        let best_grade = sheets.iter().max_by_key(|&sh| sh.grade).unwrap().grade;
        let possible_answers = AnnealingSolver::solve_mcq_test(&mcq, sheets);
        time += now.elapsed().as_nanos() as f64 / 1_000_000.;

        iter += 1;
        if possible_answers.grade as usize == NUMBER_OF_QUESTIONS {
            _successes += 1;
        }
        println!(
            "My grade : {1}/{0} \nBest grade of {2} answer sheets : {3}/{0}\nPossible choices : {4}\nTime required : {5} ms", 
            NUMBER_OF_QUESTIONS, 
            possible_answers.grade, 
            SIMULATION_SHEETS, 
            best_grade,
            POSSIBLE_ANSWERS,
            time
        );
    }

    // println!("Solved {}/{} MCQ's in {:.2e} seconds", successes, iter, time / 1_000.);
}
