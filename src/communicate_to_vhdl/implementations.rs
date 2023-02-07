use std::io::{Error, ErrorKind};
use crate::communicate_to_vhdl::Communicator;
use super::Vhdlizable;

impl Vhdlizable for i32{
    fn get_necessary_bits() -> usize {
        32
    }

    fn get_bit_representation(&self) -> Vec<bool> {

        let mut v = Vec::new();

        let mut mask: i32 = 1;

        // just converting the number to binary
        loop {
            v.push(
                mask&*self != 0
            );

            mask <<= 1;

            if mask == 0{
                break
            }

        }
        v

    }

    fn construct_from_bits(v: &[bool]) -> Result<Self,Error> {
        if v.len() != 32{
            return Err(Error::new(ErrorKind::Other, "Length of input incompatible with length of output"));
        };

        let mut ret = 0;

        let mut mask = 1;

        for (i,n) in v.iter().enumerate(){
            if *n {
                ret |= mask;
            }
            mask <<= 1;
        }

        Ok(ret)
    }

    fn get_vhd_construction_code(variable_name: &str, start_index: usize) -> String {
        format!("{variable_name} <= signed(data_in({} downto {start_index}));\n",start_index+31)
    }

    fn get_vhd_declaration_code(variable_name: &str) -> String {
        format!("signal {variable_name}: signed(31 downto 0);\n")
    }

    fn get_vhd_deconstruction_code(variable_name: &str, start_index: usize) -> String {
        format!("data_out({} downto {start_index}) <= std_logic_vector({variable_name});\n",start_index+31)
    }
}


#[test]
fn test_stringify(){

    trait DebugPus{
        fn print(&self){
        }
    }

    impl DebugPus for i32{
        fn print(&self) {
            println!("the variable: {} has value: {}",stringify!(*self),*self)
        }
    }

    let prova = 5;

    prova.print();

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