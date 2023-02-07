use std::io::{Error, ErrorKind};
use crate::communicate_to_vhdl::Communicator;
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

        for (i,n) in v.iter().enumerate(){
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

        format!("{variable_name} <= {sign}(data_in({} downto {start_index}));\n",start_index+31)
    }

    fn get_vhd_declaration_code(variable_name: &str) -> String {

        let sign = match T::is_signed() {
            true => "signed",
            false => "unsigned"
        };

        format!("signal {variable_name}: {sign}(31 downto 0);\n")
    }

    fn get_vhd_deconstruction_code(variable_name: &str, start_index: usize) -> String {

        format!("data_out({} downto {start_index}) <= std_logic_vector({variable_name});\n",start_index+31)
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





#[test]
fn test(){

    use num_traits::PrimInt;

    let n = 0xF00Du16;

    fn print_second_bit<T: PrimInt>(dato: T){

        let mask = T::one().rotate_left(1);

        println!("{}",mask&dato != T::zero())

    }

    print_second_bit(10);


}

#[test]
fn middle_point_test(){

    #[derive(Vhdlizable,Debug)]
    struct Point{
        x: i32,
        y: i32
    }
    impl Point{
        fn new(x: i32,y: i32) -> Self{
            Point{
                x,
                y
            }
        }
    }

    #[derive(Vhdlizable,Debug)]
    struct Rectangle{
        p1: Point,
        p2: Point,
    }

    let rectangle = Rectangle{
        p1: Point::new(0,0),
        p2: Point::new(10,4)
    };


    //Communicator::<Rectangle,Point>::generate_vhdl_code();
    //return;

    let mut calcolate_middle_point = Communicator::<Rectangle,Point>::new_from_serial_port("COM5").unwrap();

    let middle_point = calcolate_middle_point.calculate(rectangle).unwrap();

    println!("the middle point of the rectange  is {:?}",middle_point)


}