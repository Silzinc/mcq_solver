#[derive(Clone, Debug)]
pub struct AnnealingParameters {
    pub number_of_questions: usize,
    pub simulation_sheets: usize,
    pub possible_answers: u8,
    pub starting_beta: f64,
    pub max_beta: f64,
    pub lambda_inv: f64,
}
