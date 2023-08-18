use crate::{mcq::*, parameters::*};
use rand::{thread_rng, Rng};

pub struct GuessMCQ {
    pub answers: [Answer; NUMBER_OF_QUESTIONS],
}

pub struct AnnealingSolver {
    guess  : GuessMCQ,
    sheets : [Sheet; SIMULATION_SHEETS],
    beta: f64,
    potential: u32,
}

impl GuessMCQ {
    #[cfg(not(debug_assertions))]
    fn grade(&self, s: &Sheet) -> u8 {
        let mut grade = 0u8;
        for k in 0..NUMBER_OF_QUESTIONS {
            grade += (self.answers[k] == s.answers[k]) as u8;
        }
        grade
    }

    #[allow(non_snake_case)]
    #[cfg(debug_assertions)]
    fn grade(&self, s: &Sheet) -> u8 { // call this one `grade_simd`
        use std::simd::{Simd, SimdPartialEq, ToBitMask};
        // Max number of lanes for a SIMD is 64
        let self__simd1: Simd<Answer, 64> = Simd::from_slice(&self.answers[..64]);
        let self__simd2: Simd<Answer, 64> = Simd::from_slice(&self.answers[64..]);
        let sheet_simd1: Simd<Answer, 64> = Simd::from_slice(&s   .answers[..64]);
        let sheet_simd2: Simd<Answer, 64> = Simd::from_slice(&s   .answers[64..]);

        (self__simd1.simd_eq(sheet_simd1).to_bitmask().count_ones() +
         self__simd2.simd_eq(sheet_simd2).to_bitmask().count_ones()) as u8
    }

    // `grade_simd` is supposedly around 4x faster, but llvm's automatic loop vectorizations +
    // removing overflow checks make `grade` catch up to it and even surpass it by a margin

    // count up to 50 million comparisons per second with these optimizations
    // `grade_simd` is still preferable when making unoptimized dev builds, but
    // a release build should use `grade`
}

#[allow(dead_code)]
fn cross_log(x: u8, y: u8) -> f64 {
    if x == 0 {
        0.
    } else {
        (x as f64) * (y as f64).ln()
    }
}

impl AnnealingSolver {

    fn update_potential(&mut self) {
        let mut potential = 0u32;
        for sheet in self.sheets.iter() {
            let dif = sheet.grade as i32 - self.guess.grade(&sheet) as i32;
            potential += (dif * dif) as u32;
        }
        self.potential = potential;
    }

    fn toggle_random(&mut self) -> (usize, Answer) {
        let mut rng = thread_rng();
        let index: usize = rng.gen_range(0..NUMBER_OF_QUESTIONS);
        let previous = self.guess.answers[index];
        self.guess.answers[index] = rng.gen_range(0..POSSIBLE_ANSWERS);
        (index, previous)
    }

    fn annealing(&mut self) {
        let mut rng = thread_rng();
        while self.beta < MAX_BETA {
            let old_potential = self.potential;
            let (index, previous) = self.toggle_random();
            self.update_potential();
            if self.potential == 0 {break;}

            if self.potential > old_potential {
                if rng.gen::<f64>() > (- ((self.potential - old_potential) as f64) * self.beta).exp() {
                    self.potential = old_potential;
                    self.guess.answers[index] = previous;
                } else {
                    self.beta *= LAMBDA_INV;
                }
            }
        }
    }

    fn init(sheets: [Sheet; SIMULATION_SHEETS]) -> AnnealingSolver {
        let guess = GuessMCQ {answers: sheets.iter().max_by_key(|&sh| sh.grade).unwrap().answers};

        let mut potential = 0u32;
        for sheet in sheets.iter() {
            let dif = sheet.grade as i32 - guess.grade(&sheet) as i32;
            potential += (dif * dif) as u32;
        }
        
        Self {
            guess: guess,
            sheets: sheets,
            beta: STARTING_BETA,
            potential: potential,
        }
    }

    pub fn solve_mcq(mcq: &MCQ, sheets: [Sheet; SIMULATION_SHEETS]) -> Sheet {
        let mut solver = Self::init(sheets);
        solver.annealing();

        Sheet {
            answers: solver.guess.answers,
            grade: mcq.grade(&solver.guess),
        }
    }
}