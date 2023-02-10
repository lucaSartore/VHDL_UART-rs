use std::io::Error;
use vhdl_uart_rs::communicate_to_vhdl::{Communicator, Vhdlizable};

/// the usb port the nexis is connected to
pub const USB_PORT: &str = "COM5";


// here you can find a set of basic examples of the librart


fn main() {

    assert_eq!(plus_one(100).unwrap(),101);

    assert_eq!(sum(100_000,334).unwrap(),100_334);

    assert_eq!(max(10456,25544,1000).unwrap(),25544);

    assert_eq!(complex_struct().unwrap(),());

}


/// add 1 to a number using the board
// remember to add:
//
//      output <= input + 1;
//
// to the VHDL generated code
fn plus_one(x: i32) -> Result<i32,Error>{

    // if you nead the sourcecode uncomment this line
    //let _ = Communicator::<i32,i32>::generate_vhdl_code()?;

    let mut plus_one = Communicator::<i32,i32>::new_from_serial_port(USB_PORT)?;

    let result = plus_one.calculate(&x)?;

    return Ok(result);
}


/// summ 2 numbers
// remember to add:
//
//      output <= input_x1+input_x2;
//
// to the VHDL generated code
fn sum(x1:u64, x2: u64) -> Result<u64,Error>{
    let mut sum = Communicator::<_,u64>::new_from_serial_port(USB_PORT)?;

    //sum.generate_vhdl_code_from_instance()?;

    //note: right now (fabruary 2023) the compact! macro works only in nigtly rust
    //if tou need to send many data without this macro you can create a struct and derive
    //the Vhdlizable trait (see example bellow)
    let result = sum.calculate(&compact!(x1,x2))?;

    return Ok(result);
}


/// this function find the max value of 3 numbers
// remember to add:
//
//      output <= input_x1+input_x2;
//
// to the VHDL generated code
fn max(n1: i32, n2: i32, n3: i32) -> Result<i32,Error>{


    let mut max = Communicator::<_,i32>::new_from_serial_port(USB_PORT)?;

    //max.generate_vhdl_code_from_instance()?;

    //note: right now (fabruary 2023) the compact! macro works only in nigtly rust
    //if tou need to send many data without this macro you can create a struct and derive
    //the Vhdlizable trait (see example bellow)
    let result = max.calculate(&compact!(n1,n2,n3))?;

    return Ok(result);
}

/// an examole of a complex struct been used
// remember to add:
//
//     output_point_1_x            <= input_point_1_x;
//     output_point_1_y            <= input_point_1_y;
//     output_point_2_x            <= input_point_2_x;
//     output_point_2_y            <= input_point_2_y;
//     output_point_3_x            <= input_point_3_x;
//     output_point_3_y            <= input_point_3_y;
//     output_fill_color_red       <= input_fill_color_red;
//     output_fill_color_green     <= input_fill_color_green;
//     output_fill_color_blue      <= input_fill_color_blue;
//
// to the VHDL generated code
#[allow(dead_code)]
fn complex_struct() -> Result<(),Error>{

    #[derive(Vhdlizable)]
    #[derive(Default,PartialEq,Debug)]
    struct Color{
        red: u8,
        green: u8,
        blue: u8,
    }

    #[derive(Vhdlizable)]
    #[derive(Default,PartialEq,Debug)]
    struct Point{
        x: i32,
        y: i32
    }

    #[derive(Vhdlizable)]
    #[derive(Default,PartialEq,Debug)]
    struct Triangle{
        point_1: Point,
        point_2: Point,
        point_3: Point,
        fill_color: Color
    }

    //Communicator::<Triangle,Triangle>::generate_vhdl_code();

    let mut t = Triangle::default();
    t.point_2.y = 10;
    t.point_3.x = -4;
    t.fill_color.blue = 255;


    let mut sum = Communicator::<Triangle,Triangle>::new_from_serial_port(USB_PORT)?;

    let new_t = sum.calculate(&t)?;

    assert_eq!(new_t,t);

    return Result::Ok(())
}



#[macro_export]
// this macro si a little strange, it only works in nigtly rust, and is not possible to put it
// in the main
macro_rules! compact {
    ( $a:ident , $b:ident) => {
        {
            #[derive(Vhdlizable)]
            struct Compact<A,B>{
                $a: A,
                $b: B
            }

            Compact{
                $a: $a,
                $b: $b
            }
        }
    };
    ( $a:ident , $b:ident ,  $c:ident) => {
        {
            #[derive(Vhdlizable,Debug)]
            struct Compact<A,B,C>{
                $a: A,
                $b: B,
                $c: C
            }

            Compact{
                $a: $a,
                $b: $b,
                $c: $c
            }
        }
    };
        ( $a:ident , $b:ident ,  $c:ident , $d:ident) => {
        {
            #[derive(Vhdlizable,Debug)]
            struct Compact<A,B,C,D>{
                $a: A,
                $b: B,
                $c: C,
                $d: D
            }

            Compact{
                $a: $a,
                $b: $b,
                $c: $c,
                $d: $d,
            }
        }
    };
}