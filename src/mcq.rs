use crate::{parameters::AnnealingParameters, sheet::*, vec_util::vec_from_fn};
use rand::{thread_rng, Rng};

pub struct MCQ<'a> {
    pub answers: Vec<Answer>,
    pub params: &'a AnnealingParameters,
}

impl<'a> MCQ<'a> {
    pub fn generate_random<'b: 'a>(params: &'b AnnealingParameters) -> Self {
        let mut rng = thread_rng();
        Self {
            answers: vec_from_fn(params.number_of_questions, || {
                rng.gen_range(0..params.possible_answers)
            }),
            params: params,
        }
    }

    pub fn generate_random_sheet(&self) -> Sheet {
        let mut rng = thread_rng();
        let answers: Vec<Answer> = vec_from_fn(self.params.number_of_questions, || {
            rng.gen_range(0..self.params.possible_answers)
        });
        let mut grade = 0u8;
        for k in 0..self.params.number_of_questions {
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
        for k in 0..self.params.number_of_questions {
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
