use crate::{
    mcq::{Answer::*, *},
    bool_tables::BoolTable
};

const MAX_PROCESSED_SHEETS: usize = 17;
// We take 16 sheets so that the match_code of a QuestionsClass can fit in a u16 integer

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct QuestionsClass {
    questions: &'static [u8], // Concerned questions numbers
    match_code: BoolTable,   // Informs on which sheets agree with the answers of the questions
    reference: Sheet,
    grade: u8,
}

impl QuestionsClass {
    fn broken_by(&self, s1: &Sheet) -> bool {
        let s2 = &self.reference;
        // Check if s1 and s2 either always agree or always disagree on the questions of self

        let toggle = s1.answers[self.questions[0] as usize] == s2.answers[self.questions[0] as usize];
        for question in self.questions[1..].iter() {
            if toggle ^ (s1.answers[*question as usize] == s2.answers[*question as usize]) {
                return true;
            }
        }
        return false;
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct ClassesScheme {
    classes: [Option<QuestionsClass>; NUMBER_OF_QUESTIONS],
    num_classes: u8,
}

impl ClassesScheme {
    fn count_classes_gained(&self, s: &Sheet) -> u8 {
        let mut g = 0u8;
        for class in self.classes[0..(self.num_classes as usize)].iter() {
            match class {
                Some(ref qc) => g += qc.broken_by(s) as u8,
                None => unreachable!(),
            }
        }
        g
    }
}

#[allow(dead_code)]
pub struct Solver {
    mcq: MCQ,
    sheets: [Option<Sheet>; MAX_PROCESSED_SHEETS],
    num_sheets: u8, // number of processed sheets

    // A class scheme is None if it hasn't been determined yet
    classes_schemes: Box<[Option<ClassesScheme>; 1 << MAX_PROCESSED_SHEETS]>,
    // The user needs to be warry of the data allocated on the heap
    // With MAX_PROCESSED_SHEETS equal to 17, a solver weighs around 2 MB on the heap
}

#[allow(dead_code)]
impl Solver {
    const INIT_SHEET: Option<Sheet> = None;
    const INIT_CS: Option<ClassesScheme> = None;

    pub fn init_random() -> Self {
        Self {
            mcq: MCQ::generate_random(),
            sheets: [Self::INIT_SHEET; MAX_PROCESSED_SHEETS],
            num_sheets: 0u8,
            classes_schemes: Box::new([Self::INIT_CS; 1 << MAX_PROCESSED_SHEETS])
        }
    }

    
    fn get_finest_scheme(&self) -> &ClassesScheme {
        let mut index = 0;
        for _ in 0..self.num_sheets {
            index = (index << 1) | 1;
        }
        self.classes_schemes[index].as_ref().unwrap()
        // as_ref is here because of how Rust handles heap memory
        // unwrap converts Some(x) to x without writing a verbose match
    }

    fn pick_sheet(&self, sheets: &'static [Sheet]) -> &'static Sheet {
        // We can take static lifetimes for the borrow of a Sheet since the solver exists for the whole execution
        // and so do the sheets
        if sheets.len() == 0 {
            panic!("No sheet to pick")
        }
        let mut sheet_to_pick = &sheets[0];
        if self.num_sheets == 0 {
            sheet_to_pick // There is no need to look for another one
        } else {
            let finest_scheme = self.get_finest_scheme();
            let mut g = finest_scheme.count_classes_gained(&sheet_to_pick);

            for sheet in sheets[1..].iter() {
                if g == finest_scheme.num_classes {
                    return sheet_to_pick; // cannot do better
                }

                let new_g = finest_scheme.count_classes_gained(&sheet);
                if new_g > g {
                    g = new_g;
                    sheet_to_pick = &sheet;
                }
            }

            sheet_to_pick
        }   
    }
}