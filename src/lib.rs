#[allow(unused_imports)]
use pyo3::{
    prelude::*,
    types::*,
};

mod vec_util;
mod parameters;
mod sheet;
mod annealing;
mod parsing;
/*
mod test {
    pub mod mcq;
}
*/


use parsing::py_solve_mcq_back;

#[pyfunction]
pub fn py_solve_mcq_front(
    path_to_grades: String,
    path_to_sheets: Vec<String>,
    grades_separator: char,
    number_of_questions: usize,
    starting_beta: f64,
    max_beta: f64,
    lambda_inv: f64,
    answer_tokens: Vec<char>,
) -> Py<PyAny> {
    match py_solve_mcq_back(
        path_to_grades,
        path_to_sheets,
        grades_separator,
        number_of_questions,
        starting_beta,
        max_beta,
        lambda_inv,
        answer_tokens,
    ) {
        Ok(ans) => Python::with_gil(|py: Python| {
            ans.to_object(py)
        }),
        Err(s) => Python::with_gil(|py: Python| {
            s.to_object(py)
        })
    }
}

#[pymodule]
fn mcq_solver_rustlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_solve_mcq_front, m)?)?;
    Ok(())
}