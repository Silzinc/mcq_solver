pub type Answer = u8;

#[derive(Clone, Debug)]
pub struct Sheet {
    pub answers: Vec<Answer>,
    pub grade: u8,
}
