use crate::{
    annealing::AnnealingSolver, parameters::AnnealingParameters, sheet::*, vec_util::vec_from_fn,
};
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
}

impl<'a> AnnealingSolver<'a> {
    // Testing purposes
    pub fn solve_mcq_test<'b: 'a>(mcq: &MCQ<'b>) -> bool {
        let sheets: Vec<Sheet> =
            vec_from_fn(mcq.params.simulation_sheets, || mcq.generate_random_sheet());
        let answers = Self::solve_mcq(sheets, mcq.params);
        for (try_answer, true_answer) in std::iter::zip(answers.iter(), mcq.answers.iter()) {
            if try_answer != true_answer {
                return false;
            }
        }
        true
    }
}
