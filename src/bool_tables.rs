// Space-optimized boolean table in which a boolean weighs 1 bit
#![allow(dead_code)]

#[allow(non_camel_case_types)]
pub type container_uint = u16; // Max size is 16 booleans
pub const SIZE_OF_BOOLTABLE: usize = std::mem::size_of::<container_uint>() * 8;

#[derive(Clone, Copy)]
pub struct BoolTable(container_uint); 

use std::fmt;
impl fmt::Debug for BoolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_bool_array())
    }
}

impl BoolTable {
    pub fn init() -> Self {
        BoolTable(0)
    }

    pub fn get(&self, index: u8) -> bool {
        self.0 >> index & 1 != 0
    }

    pub fn set(&mut self, index: u8, value: bool) {
        if value {self.0 |= 1 << index}
        else {self.0 &= (1 << index) ^ container_uint::MAX}
    }

    pub fn toggle(&mut self, index: u8) {
        self.0 ^= 1 << index
    }

    fn to_bool_array(&self) -> [bool; SIZE_OF_BOOLTABLE] {
        let mut array = [false; SIZE_OF_BOOLTABLE];
        let mut num = self.0;
        for k in 0..SIZE_OF_BOOLTABLE {
            array[k] = num & 1 != 0;
            num >>= 1;
        }
        array
    }
}