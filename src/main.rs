mod annealing;
mod test {
    pub mod mcq;
}
mod parameters;
mod sheet;
mod vec_util;
// mod io;

use annealing::AnnealingSolver;
use parameters::*;
use test::mcq::MCQ;

fn main() {
    let params = AnnealingParameters::default();
    let mut _time = 0f64;
    let mut iter = 0u32;
    let mut _successes = 0u32;

    while iter < 1u32 {
        let mcq = MCQ::generate_random(&params);
        iter += 1;
        let now = std::time::Instant::now();
        if AnnealingSolver::solve_mcq_test(&mcq) {
            _successes += 1;
            println!(
                "My grade : {0}/{0} \n{1} answer sheets\nPossible choices : {2}\nTime required : {3} ms", 
                params.number_of_questions,  
                params.simulation_sheets, 
                params.possible_answers,
                now.elapsed().as_nanos() as f64 / 1_000_000.
            );
        }
        _time += now.elapsed().as_nanos() as f64 / 1_000_000.;
    }

    // println!("Solved {}/{} MCQ's in {:.2e} seconds", successes, iter, time / 1_000.);
}
