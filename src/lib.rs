#[allow(unused_imports)]
use pyo3::{prelude::*, types::*};

mod annealing;
mod basic_solve;
mod parameters;
mod parsing;
mod sheet;
mod vec_util;

use basic_solve::solve_mcq_back;
use parsing::solve_mcq_from_files_back;

#[pyfunction]
pub fn _solve_from_files(
	path_to_sheets: Vec<String>,
	answer_tokens: Vec<char>,
	path_to_grades: String,
	grades_separator: char,
	number_of_questions: usize,
	starting_beta: f64,
	max_beta: f64,
	lambda_inv: f64,
) -> Py<PyAny>
{
	match solve_mcq_from_files_back(
		path_to_sheets,
		answer_tokens,
		path_to_grades,
		grades_separator,
		number_of_questions,
		starting_beta,
		max_beta,
		lambda_inv,
	)
	{
		Ok(ans) => Python::with_gil(|py: Python| {
			(ans, true).to_object(py) // The boolean indicates if there was an error
		}),
		Err(s) => Python::with_gil(|py: Python| (s, false).to_object(py)),
	}
}

#[pyfunction]
pub fn _solve(
	sheets_raw: Vec<Vec<char>>,
	answer_tokens: Vec<char>,
	grades: Vec<u8>,
	starting_beta: f64,
	max_beta: f64,
	lambda_inv: f64,
) -> Py<PyAny>
{
	match solve_mcq_back(
		sheets_raw,
		answer_tokens,
		grades,
		starting_beta,
		max_beta,
		lambda_inv,
	)
	{
		Ok(ans) => Python::with_gil(|py: Python| (ans, true).to_object(py)),
		Err(s) => Python::with_gil(|py: Python| (s, false).to_object(py)),
	}
}

#[pymodule]
fn mcq_solver(_py: Python, m: &PyModule) -> PyResult<()>
{
	m.add_function(wrap_pyfunction!(_solve_from_files, m)?)?;
	m.add_function(wrap_pyfunction!(_solve, m)?)?;
	Ok(())
}
