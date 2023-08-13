pub const NUMBER_OF_QUESTIONS: usize = 30;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Answer {
    A,
    B,
    // More ?
}
use Answer::{A, B};

impl std::ops::Not for Answer {
    type Output = Self;

    fn not(self) -> Self {
        if self == A {B} else {A}
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Sheet {
    pub answers: [Answer; NUMBER_OF_QUESTIONS],
    pub grade: u8,
}

impl Sheet {
    pub fn generate_random(mcq: &MCQ) -> Self {
        let mut answers = [A; NUMBER_OF_QUESTIONS];
        let mut grade = 0u8;
        for k in 0..NUMBER_OF_QUESTIONS {
            if rand::random() {
                answers[k] = B;
            }
            grade += (answers[k] == mcq.0[k]) as u8;
        }
        Self {
            answers: answers,
            grade: grade,
        }
    }
}

impl std::default::Default for Sheet {
    fn default() -> Self {
        Self {
            answers: [A; NUMBER_OF_QUESTIONS],
            grade: 0u8,
        }
    }
}

pub struct MCQ([Answer; NUMBER_OF_QUESTIONS]);

impl MCQ {
    pub fn generate_random() -> Self {
        let mut answers = [A; NUMBER_OF_QUESTIONS];
        for k in 0..NUMBER_OF_QUESTIONS {
            if rand::random() {
                answers[k] = B;
            }
        }
        MCQ(answers)
    }
}