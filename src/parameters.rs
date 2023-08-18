/* Parameters */
pub const NUMBER_OF_QUESTIONS: usize = 128 ; // Chosen as a power of 2 on purpose to use SIMD
pub const SIMULATION_SHEETS  : usize = 800 ;
pub const POSSIBLE_ANSWERS   : u8    = 8   ;
pub const STARTING_BETA      : f64   = 0.1 ;
pub const MAX_BETA           : f64   = 0.5 ;
pub const LAMBDA_INV         : f64   = 1.01;