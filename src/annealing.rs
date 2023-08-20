use crate::{
    parameters::AnnealingParameters, 
    sheet::{Answer, Sheet},
};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct CandidateMCQ {
    pub answers: Vec<Answer>,
}

pub struct AnnealingSolver<'a> {
    pub current_candidate: CandidateMCQ,
    pub best_candidate: CandidateMCQ,
    pub sheets: Vec<Sheet>,
    pub beta: f64,
    pub potential: u32,
    pub params: &'a AnnealingParameters,
}

impl CandidateMCQ {
    fn grade(&self, s: &Sheet) -> u8 {
        let mut grade = 0u8;
        for k in 0..self.answers.len() {
            grade += (self.answers[k] == s.answers[k]) as u8;
        }
        grade
    }

    fn copy_from(&mut self, other: &Self) {
        self.answers.copy_from_slice(other.answers.as_slice());
    }
}

// Maybe later used to replace least squares with cross entropy
#[allow(dead_code)]
fn cross_log(x: u8, y: u8) -> f64 {
    if x == 0 {
        0.
    } else {
        (x as f64) * (y as f64).ln()
    }
}

impl<'a> AnnealingSolver<'a> {
    fn update_potential(&mut self) {
        let mut potential = 0u32;
        for sheet in self.sheets.iter() {
            let dif = sheet.grade as i32 - self.current_candidate.grade(&sheet) as i32;
            potential += (dif * dif) as u32;
        }
        self.potential = potential;
    }

    fn toggle_random(&mut self) -> (usize, Answer) {
        let mut rng = thread_rng();
        let index: usize = rng.gen_range(0..self.params.number_of_questions);
        let previous = self.current_candidate.answers[index];
        self.current_candidate.answers[index] = rng.gen_range(0..self.params.possible_answers);
        (index, previous)
    }

    fn annealing(&mut self) {
        let mut rng = thread_rng();
        while self.beta < self.params.max_beta {
            let old_potential = self.potential;
            let (index, previous) = self.toggle_random();
            self.update_potential();
            if self.potential == 0 {
                break;
            }

            if self.potential > old_potential {
                if rng.gen::<f64>() > (-((self.potential - old_potential) as f64) * self.beta).exp()
                {
                    self.potential = old_potential;
                    self.current_candidate.answers[index] = previous;
                } else {
                    self.beta *= self.params.lambda_inv;
                }
            }
        }
    }

    fn init<'b: 'a>(sheets: Vec<Sheet>, params: &'b AnnealingParameters) -> Self {
        let current_candidate = CandidateMCQ {
            answers: Vec::from(
                sheets
                    .iter()
                    .max_by_key(|sh| sh.grade)
                    .unwrap()
                    .answers
                    .clone(),
            ),
        };

        let mut potential = 0u32;
        for sheet in sheets.iter() {
            let dif = sheet.grade as i32 - current_candidate.grade(&sheet) as i32;
            potential += (dif * dif) as u32;
        }

        Self {
            current_candidate: current_candidate,
            sheets: sheets,
            beta: params.starting_beta,
            potential: potential,
            params: params,
        }
    }
    
    pub fn solve_mcq<'b: 'a>(sheets: Vec<Sheet>, params: &'b AnnealingParameters) -> Vec<Answer> {
        let mut solver = Self::init(sheets, params);
        solver.annealing();
        solver.current_candidate.answers
    }
}
