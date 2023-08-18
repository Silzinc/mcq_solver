#![feature(portable_simd)]

mod mcq;
mod annealing;
mod parameters;

use mcq::{Sheet, MCQ};
use annealing::AnnealingSolver;
use parameters::{SIMULATION_SHEETS, NUMBER_OF_QUESTIONS};

fn main() {
    let mut time = 0f64;
    let mut iter = 0u32;
    let mut successes = 0u32;

    while iter < 5_000u32 {
        let mcq = MCQ::generate_random();
        let sheets: [Sheet; SIMULATION_SHEETS] = core::array::from_fn(|_| mcq.generate_random_sheet());

        let now = std::time::Instant::now();
        // let best_grade = sheets.iter().max_by_key(|&sh| sh.grade).unwrap().grade;
        let possible_answers = AnnealingSolver::solve_mcq(&mcq, sheets);
        time += now.elapsed().as_nanos() as f64 / 1_000_000.;

        iter += 1;
        if possible_answers.grade as usize == NUMBER_OF_QUESTIONS {
            successes += 1;
        }
        /*
        println!(
            "My grade : {1}/{0} \nBest grade of {2} answer sheets : {3}/{0}", 
            NUMBER_OF_QUESTIONS, 
            possible_answers.grade, 
            SIMULATION_SHEETS, 
            best_grade
        );
        */
    }

    println!("Solved {}/{} MCQ's in {:.2e} seconds", successes, iter, time / 1_000.);
}
