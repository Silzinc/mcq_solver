#![allow(unused_imports)]

mod bool_tables;
use bool_tables::BoolTable;

mod mcq;
use mcq::*;
use Answer::*;

mod solver;

fn main() {
    let mcq = MCQ::generate_random();
    for k in 0..100 {
        if Sheet::generate_random(&mcq).grade == mcq::NUMBER_OF_QUESTIONS as u8 {
            println!("Max grade found at sheet {}/100", k);
            break;
        }
    }

    // let _array = [[0u8; 24]; 1 << 18];
    // println!("{} bytes allocated on a big array\n{} bytes remaining on the stack", 
    // std::mem::size_of_val(&_array), stacker::remaining_stack().unwrap());
    println!("{}", std::mem::size_of::<Option<[i32; 2]>>());
    println!("Solver weighs {}/{} available bytes", 
    std::mem::size_of::<solver::Solver>(), stacker::remaining_stack().unwrap());
}
