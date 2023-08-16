mod annealing;
mod mcq;
mod parameters;
mod sheet;
mod vec_util;
// mod io;

use annealing::AnnealingSolver;
use mcq::MCQ;
use parameters::*;
use sheet::Sheet;
use vec_util::vec_from_fn;

fn main() {
    let params = AnnealingParameters::default();
    let mut _time = 0f64;
    let mut iter = 0u32;
    let mut _successes = 0u32;

    while iter < 1u32 {
        let mcq = MCQ::generate_random(&params);
        let sheets: Vec<Sheet> =
            vec_from_fn(params.simulation_sheets, || mcq.generate_random_sheet());
        let best_grade = sheets.iter().max_by_key(|sh| sh.grade).unwrap().grade;
        iter += 1;
        let now = std::time::Instant::now();
        if AnnealingSolver::solve_mcq_test(&mcq, sheets) {
            _successes += 1;
            println!(
                "My grade : {0}/{0} \nBest grade of {1} answer sheets : {2}/{0}\nPossible choices : {3}\nTime required : {4} ms", 
                params.number_of_questions,  
                params.simulation_sheets, 
                best_grade,
                params.possible_answers,
                now.elapsed().as_nanos() as f64 / 1_000_000.
            );
        }
        _time += now.elapsed().as_nanos() as f64 / 1_000_000.;
    }

    // println!("Solved {}/{} MCQ's in {:.2e} seconds", successes, iter, time / 1_000.);
}
