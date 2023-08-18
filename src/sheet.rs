use crate::parameters::NUMBER_OF_QUESTIONS;

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