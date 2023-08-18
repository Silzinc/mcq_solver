use crate::{
    annealing::AnnealingSolver,
    parameters::*,
    sheet::{Answer, Sheet},
};
use std::{collections::HashMap, fs};

struct AnnealingParser<'a> {
    path_to_grades: String,
    path_to_sheets: Vec<String>,
    grades_separator: char,
    annealing_params: &'a AnnealingParameters,
    answer_tokens_map: HashMap<char, Answer>,
}

impl<'a> AnnealingParser<'a> {

    fn parse_into_grades(
        &self
    ) -> Result<Vec<u8>, String> {
        let input = match fs::read_to_string(&self.path_to_grades) {
            Ok(s) => s,
            Err(_) => return Err(format!("Parsing grades: Failed to load {}", self.path_to_grades)),
        };
        let separator_token = self.grades_separator;
        let params = &self.annealing_params;

        if separator_token.is_digit(10) {
            return Err(
                "Grades parsing: The separator token is a digit and therefore invalid".to_string(),
            );
        }

        let mut grades = Vec::<u8>::with_capacity(params.simulation_sheets);
        let mut current_grade = 0u8;

        for token in input
            .chars()
            .filter(|&c| c.is_digit(10) || c == separator_token)
        {
            if token == separator_token {
                grades.push(current_grade);
                current_grade = 0u8;
            } else if grades.len() == params.simulation_sheets {
                return Err(format!(
                    "Grades parsing: Too many grades were found in the input string (expected {})",
                    params.simulation_sheets
                ));
            } else {
                let digit = token.to_digit(10).unwrap();
                if 10 * current_grade as u32 + digit > params.number_of_questions as u32 {
                    return Err(format!(
                        "Grades parsing: The grade {}{} is invalid because above {}",
                        current_grade, digit, params.number_of_questions
                    ));
                }
                current_grade *= 10;
                current_grade += digit as u8;
            }
        }

        if grades.len() < params.number_of_questions {
            Err(format!("Grades parsing: Not enough grades were found in the input string (expected {}, found {})",
            params.number_of_questions, grades.len()))
        } else {
            Ok(grades)
        }
    }

    fn parse_into_answers(
        &self,
        index: usize,
    ) -> Result<Vec<Answer>, String> {
        let input = match fs::read_to_string(&self.path_to_sheets[index]) {
            Ok(s) => s,
            Err(_) => return Err(format!("Parsing answers: failed to load {}", self.path_to_sheets[index])),
        };
        let answer_tokens = &self.answer_tokens_map;
        let params = &self.annealing_params;
        let mut answers = Vec::<Answer>::with_capacity(params.number_of_questions);

        for token in input.chars().filter(|c| answer_tokens.contains_key(c)) {
            if answers.len() == params.number_of_questions {
                return Err(format!(
                    "Parsing answers: Too many answers were found in the input string (expected {})",
                    params.number_of_questions
                ));
            }
            answers.push(*answer_tokens.get(&token).unwrap());
        }

        if answers.len() < params.number_of_questions {
            Err(format!("Parsing answers: Not enough answers were found in the input string (expected {}, found {})",
                params.number_of_questions, answers.len()))
        } else {
            Ok(answers)
        }
    }
}

pub fn solve_mcq_from_files_back(
    path_to_sheets: Vec<String>,
    answer_tokens: Vec<char>,
    path_to_grades: String,
    grades_separator: char,

    // annealing_params :
    number_of_questions: usize,
    // simulation_sheets: usize, // can be found in path_to_sheets length
    // possible_answers: u8, // can be found in answer_tokens length
    starting_beta: f64,
    max_beta: f64,
    lambda_inv: f64,
) -> Result<Vec<char>, String> {

    let simulation_sheets = path_to_sheets.len();
    let possible_answers = answer_tokens.len() as u8;
    let annealing_params = 
    AnnealingParameters {
        number_of_questions,
        simulation_sheets,
        possible_answers,
        starting_beta,
        max_beta,
        lambda_inv,
    };

    let mut answer_tokens_map = HashMap::<char, Answer>::new();
    for k in 0..possible_answers {
        answer_tokens_map.insert(answer_tokens[k as usize], k);
    }
    
    let parser = 
    AnnealingParser {
        path_to_grades,
        path_to_sheets,
        grades_separator,
        annealing_params: &annealing_params,
        answer_tokens_map,
    };

    let grades = match parser.parse_into_grades() {
        Ok(gs) => gs,
        Err(s) => return Err(s),
    };

    let mut sheets = Vec::<Sheet>::with_capacity(simulation_sheets);
    for k in 0..simulation_sheets {
        sheets.push(
        Sheet {
            answers: match parser.parse_into_answers(k) {
                Ok(ans) => ans,
                Err(s) => return Err(s),
            },
            grade: grades[k],
        });
    }

    let mut solved_answers = Vec::<char>::with_capacity(number_of_questions);
    for answer_int in AnnealingSolver::solve_mcq(sheets, &annealing_params) {
        solved_answers.push(answer_tokens[answer_int as usize]);
    }
    Ok(solved_answers)
}
