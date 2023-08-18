mod vec_util;
mod sheet;
mod mcq;
mod annealing;
mod parameters;
// mod io;

use vec_util::vec_from_fn;
use mcq::MCQ;
use sheet::Sheet;
use annealing::AnnealingSolver;
use parameters::*;

fn main() {
    let mut time = 0f64;
    let mut iter = 0u32;
    let mut _successes = 0u32;

    while iter < 1u32 {
        let mcq = MCQ::generate_random();
        let sheets: Vec<Sheet> = vec_from_fn(SIMULATION_SHEETS, || mcq.generate_random_sheet());
        let best_grade = sheets.iter().max_by_key(|sh| sh.grade).unwrap().grade;
        iter += 1;
        let now = std::time::Instant::now();
        if AnnealingSolver::solve_mcq_test(&mcq, sheets) {
            _successes += 1;
            println!(
                "My grade : {0}/{0} \nBest grade of {1} answer sheets : {2}/{0}\nPossible choices : {3}\nTime required : {4} ms", 
                NUMBER_OF_QUESTIONS,  
                SIMULATION_SHEETS, 
                best_grade,
                POSSIBLE_ANSWERS,
                now.elapsed().as_nanos() as f64 / 1_000_000.
            );
        }
        time += now.elapsed().as_nanos() as f64 / 1_000_000.;
    }

    // println!("Solved {}/{} MCQ's in {:.2e} seconds", successes, iter, time / 1_000.);
}
