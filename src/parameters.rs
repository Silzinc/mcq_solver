#[derive(Clone, Debug)]
pub struct AnnealingParameters {
    pub number_of_questions: usize,
    pub simulation_sheets: usize,
    pub possible_answers: u8,
    pub starting_beta: f64,
    pub max_beta: f64,
    pub lambda_inv: f64,
}

impl std::default::Default for AnnealingParameters {
    fn default() -> Self {
        Self {
            number_of_questions: 120,
            simulation_sheets: 500,
            possible_answers: 8,
            starting_beta: 0.1,
            max_beta: 0.5,
            lambda_inv: 1.01,
        }
    }
}
