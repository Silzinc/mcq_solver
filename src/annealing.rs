use crate::{
	parameters::AnnealingParameters,
	sheet::{Answer, Sheet},
};
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct CandidateMCQ
{
	pub answers: Vec<Answer>,
}

pub struct AnnealingSolver<'a>
{
	pub current_candidate: CandidateMCQ,
	pub current_potential: u32,
	pub best_candidate: CandidateMCQ,
	pub best_potential: u32,
	pub sheets: Vec<Sheet>,
	pub beta: f64,
	pub params: &'a AnnealingParameters,
}

impl CandidateMCQ
{
	fn grade(&self, s: &Sheet) -> u8
	{
		let mut grade = 0u8;
		for k in 0..self.answers.len() {
			grade += (self.answers[k] == s.answers[k]) as u8;
		}
		grade
	}

	fn copy_from(&mut self, other: &Self) { self.answers.copy_from_slice(other.answers.as_slice()); }
}

// Maybe later used to replace least squares with cross entropy
#[allow(dead_code)]
fn cross_log(x: u8, y: u8) -> f64
{
	if x == 0 {
		0.
	} else {
		(x as f64) * (y as f64).ln()
	}
}

impl<'a> AnnealingSolver<'a>
{
	fn update_current_potential(&mut self)
	{
		let mut current_potential = 0u32;
		for sheet in self.sheets.iter() {
			let dif = sheet.grade as i32 - self.current_candidate.grade(&sheet) as i32;
			current_potential += (dif * dif) as u32;
		}
		self.current_potential = current_potential;
	}

	fn toggle_random(&mut self) -> (usize, Answer)
	{
		let mut rng = thread_rng();
		let index: usize = rng.gen_range(0..self.params.number_of_questions);
		let previous_answer = self.current_candidate.answers[index];
		self.current_candidate.answers[index] = rng.gen_range(0..self.params.possible_answers);
		(index, previous_answer)
	}

	fn annealing(&mut self)
	{
		let mut rng = thread_rng();
		while self.best_potential != 0 && self.beta < self.params.max_beta {
			let old_current_potential = self.current_potential;
			let (index, previous_answer) = self.toggle_random();
			self.update_current_potential();

			if self.current_potential < self.best_potential {
				self.best_candidate.copy_from(&self.current_candidate);
				self.best_potential = self.current_potential;
				continue;
			}

			if self.current_potential > old_current_potential {
				if rng.gen::<f64>() > (-((self.current_potential - old_current_potential) as f64) * self.beta).exp() {
					self.current_potential = old_current_potential;
					self.current_candidate.answers[index] = previous_answer;
				} else {
					self.beta *= self.params.lambda_inv;
				}
			}
		}
	}

	fn init<'b: 'a>(sheets: Vec<Sheet>, params: &'b AnnealingParameters) -> Self
	{
		let starting_candidate = CandidateMCQ {
			answers: Vec::from(sheets.iter().max_by_key(|sh| sh.grade).unwrap().answers.clone()),
		};

		let mut starting_potential = 0u32;
		for sheet in sheets.iter() {
			let dif = sheet.grade as i32 - starting_candidate.grade(&sheet) as i32;
			starting_potential += (dif * dif) as u32;
		}

		Self {
			current_candidate: starting_candidate.clone(),
			current_potential: starting_potential,
			best_candidate: starting_candidate,
			best_potential: starting_potential,
			sheets,
			beta: params.starting_beta,
			params,
		}
	}

	pub fn solve_mcq<'b: 'a>(sheets: Vec<Sheet>, params: &'b AnnealingParameters) -> Vec<Answer>
	{
		let mut solver = Self::init(sheets, params);
		solver.annealing();
		solver.best_candidate.answers
	}
}
