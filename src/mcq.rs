use crate::parameters::{NUMBER_OF_QUESTIONS, POSSIBLE_ANSWERS};
use rand::{thread_rng, Rng};

pub type Answer = u8;

#[derive(Clone, Debug)]
pub struct Sheet {
    pub answers: [Answer; NUMBER_OF_QUESTIONS],
    pub grade: u8,
}

impl std::default::Default for Sheet {
    fn default() -> Self {
        Self {
            answers: [0 as Answer; NUMBER_OF_QUESTIONS],
            grade: 0u8,
        }
    }
}

pub struct MCQ {
    pub answers: [Answer; NUMBER_OF_QUESTIONS],
}

impl MCQ {
    pub fn generate_random() -> Self {
        let mut rng = thread_rng();
        Self {answers: core::array::from_fn(|_| rng.gen_range(0..POSSIBLE_ANSWERS))}
    }

    pub fn generate_random_sheet(&self) -> Sheet {
        let mut rng = thread_rng();
        let answers: [Answer; NUMBER_OF_QUESTIONS] = core::array::from_fn(|_| rng.gen_range(0..POSSIBLE_ANSWERS));
        let mut grade = 0u8;
        for k in 0..NUMBER_OF_QUESTIONS {
            grade += (answers[k] == self.answers[k]) as u8;
        }
        Sheet {
            answers: answers,
            grade: grade,
        }
    }

    // #[cfg(not(debug_assertions))]
    pub fn grade(&self, guess: &crate::annealing::GuessMCQ) -> u8 {
        let mut grade = 0u8;
        for k in 0..NUMBER_OF_QUESTIONS {
            grade += (self.answers[k] == guess.answers[k]) as u8;
        }
        grade
    }

    /*
    #[cfg(debug_assertions)]
    pub fn grade(&self, gues: &crate::annealing::GuessMCQ) -> u8 { // let's call this `grade_simd`
        use std::simd::{Simd, SimdPartialEq, ToBitMask};
        // Max number of lanes for a SIMD is 64
        let self_simd1: Simd<Answer, 64> = Simd::from_slice(&self.answers[..64]);
        let self_simd2: Simd<Answer, 64> = Simd::from_slice(&self.answers[64..]);
        let gues_simd1: Simd<Answer, 64> = Simd::from_slice(&gues.answers[..64]);
        let gues_simd2: Simd<Answer, 64> = Simd::from_slice(&gues.answers[64..]);

        (self_simd1.simd_eq(gues_simd1).to_bitmask().count_ones() +
         self_simd2.simd_eq(gues_simd2).to_bitmask().count_ones()) as u8
    }

    // `grade_simd` is supposedly around 4x faster, but llvm's automatic loop vectorizations +
    // removing overflow checks make `grade` catch up to it and even surpass it by a margin

    // count up to 50 million comparisons per second with these optimizations
    // `grade_simd` is still preferable when making unoptimized dev builds, but
    // a release build should use `grade`
    */
}