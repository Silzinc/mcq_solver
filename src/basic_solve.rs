use crate::{
	annealing::AnnealingSolver,
	parameters::AnnealingParameters,
	sheet::{Answer, Sheet},
};
use std::collections::HashMap;

pub fn solve_mcq_back(
	sheets_raw: Vec<Vec<char>>,
	answer_tokens: Vec<char>,
	grades: Vec<u8>,

	// annealing_params :
	// number_of_questions: usize, // can be found in sheets_raw[0] length
	// simulation_sheets: usize,   // can be found in sheets_raw length
	// possible_answers: u8,       // can be found in answer_tokens length
	starting_beta: f64,
	max_beta: f64,
	lambda_inv: f64,
) -> Result<Vec<char>, String>
{
	if sheets_raw.len() == 0
	{
		return Err("Error: No input answer sheet given".to_string());
	}
	if sheets_raw.len() != grades.len()
	{
		return Err(format!(
			"Error: The number of answer sheets ({}) is not the number of grades ({}) given",
			sheets_raw.len(),
			grades.len()
		));
	}
	let number_of_questions = sheets_raw[0].len();
	let simulation_sheets = sheets_raw.len();
	let possible_answers = answer_tokens.len() as u8;
	let annealing_params = AnnealingParameters {
		number_of_questions,
		simulation_sheets,
		possible_answers,
		starting_beta,
		max_beta,
		lambda_inv,
	};

	let mut answer_tokens_map = HashMap::<char, Answer>::new();
	for k in 0..possible_answers
	{
		answer_tokens_map.insert(answer_tokens[k as usize], k);
	}

	let mut sheets = Vec::<Sheet>::with_capacity(simulation_sheets);
	for k in 0..simulation_sheets
	{
		let sheet = &sheets_raw[k];
		if sheet.len() != number_of_questions
		{
			return Err(format!(
				"Error: The answer sheet at index {} has {} answers but the first one provided had {}",
				k,
				sheet.len(),
				number_of_questions
			));
		}
		let mut sheet_answers = Vec::<Answer>::with_capacity(number_of_questions);
		for j in 0..number_of_questions
		{
			sheet_answers.push(*answer_tokens_map.get(&sheet[j]).unwrap());
		}
		sheets.push(Sheet {
			answers: sheet_answers,
			grade: grades[k],
		});
	}

	let mut solved_answers = Vec::<char>::with_capacity(number_of_questions);
	for answer_int in AnnealingSolver::solve_mcq(sheets, &annealing_params)
	{
		solved_answers.push(answer_tokens[answer_int as usize]);
	}
	Ok(solved_answers)
}
