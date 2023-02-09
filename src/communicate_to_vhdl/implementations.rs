use std::io::{Error, ErrorKind};
use super::Vhdlizable;

use num_traits::PrimInt;



impl<T: PrimInt + SignedDetector + std::fmt::Display> Vhdlizable for T{
    fn get_necessary_bits() -> usize {
        std::mem::size_of::<T>()*8
    }

    fn get_bit_representation(&self) -> Vec<bool> {

        let mut v = Vec::new();

        let mut mask = T::one();

        // just converting the number to binary
        loop {
            v.push(
                mask&(*self) != T::zero()
            );


            mask = mask.rotate_left(1);


            if mask == T::one(){
                break
            }
        }
        v

    }

    fn construct_from_bits(v: &[bool]) -> Result<Self,Error> {

        if v.len() != Self::get_necessary_bits(){
            return Err(Error::new(ErrorKind::Other, "Length of input incompatible with length of output"));
        };

        let mut ret = T::zero();

        let mut mask = T::one();

        for n in v.iter(){
            if *n {
                ret = ret + mask;
            }
            mask = mask.rotate_left(1);
        }

        Ok(ret)
    }

    fn get_vhd_construction_code(variable_name: &str, start_index: usize) -> String {

        let sign = match T::is_signed() {
            true => "signed",
            false => "unsigned"
        };

        format!("{variable_name} <= {sign}(data_in({} downto {start_index}));\n",start_index+Self::get_necessary_bits()-1)
    }

    fn get_vhd_declaration_code(variable_name: &str) -> String {

        let sign = match T::is_signed() {
            true => "signed",
            false => "unsigned"
        };

        format!("signal {variable_name}: {sign}({} downto 0);\n",Self::get_necessary_bits()-1)
    }

    fn get_vhd_deconstruction_code(variable_name: &str, start_index: usize) -> String {

        format!("data_out({} downto {start_index}) <= std_logic_vector({variable_name});\n",start_index+Self::get_necessary_bits()-1)
    }
}


pub trait SignedDetector{
    fn is_signed() -> bool;
}

impl SignedDetector for i8 {
    fn is_signed() -> bool {
        true
    }
}
impl SignedDetector for i16 {
    fn is_signed() -> bool {
        true
    }
}
impl SignedDetector for i32 {
    fn is_signed() -> bool {
        true
    }
}
impl SignedDetector for i64 {
    fn is_signed() -> bool {
        true
    }
}
impl SignedDetector for i128 {
    fn is_signed() -> bool {
        true
    }
}
impl SignedDetector for isize {
    fn is_signed() -> bool {
        true
    }
}

impl SignedDetector for u8 {
    fn is_signed() -> bool {
        false
    }
}
impl SignedDetector for u16 {
    fn is_signed() -> bool {
        false
    }
}

impl SignedDetector for u32 {
    fn is_signed() -> bool {
        false
    }
}

impl SignedDetector for u64 {
    fn is_signed() -> bool {
        false
    }
}

impl SignedDetector for u128 {
    fn is_signed() -> bool {
        false
    }
}

impl SignedDetector for usize {
    fn is_signed() -> bool {
        false
    }
}


impl Vhdlizable for bool {
    fn construct_from_bits(data: &[bool]) -> Result<Self, Error> where Self: Sized {
        if data.len() != Self::get_necessary_bits(){
            return Err(Error::new(ErrorKind::Other, "Length of input incompatible with length of output"));
        };
        Ok(data[0])
    }
    fn get_bit_representation(&self) -> Vec<bool> {
        vec![*self]
    }
    fn get_necessary_bits() -> usize {
        1
    }
    fn get_vhd_construction_code(variable_name: &str, start_index: usize) -> String {
        format!("{variable_name} <= data_in({start_index});\n")
    }
    fn get_vhd_declaration_code(variable_name: &str) -> String {
        format!("signal {variable_name}: std_logic;\n")
    }
    fn get_vhd_deconstruction_code(variable_name: &str, start_index: usize) -> String {
        format!("data_out({start_index}) <= {variable_name};\n")
    }
}