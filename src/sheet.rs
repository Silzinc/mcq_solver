pub type Answer = u8;

#[derive(Clone, Debug)]
pub struct Sheet {
    pub answers: Vec<Answer>,
    pub grade: u8,
}

/*
impl std::default::Default for Sheet {
    fn default() -> Self {
        Self {
            answers: vec![0 as Answer; NUMBER_OF_QUESTIONS],
            grade: 0u8,
        }
    }
}
*/
