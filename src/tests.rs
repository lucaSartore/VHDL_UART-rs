use std::fmt::Debug;
#[allow(unused_imports)]
use crate::communicate_to_vhdl::{Communicator,Vhdlizable};

#[test]
fn plus_one_test(){
    //to generate the VHDL code use:
    //let _ = Communicator::<i32,i32>::generate_vhdl_code();
    //and remember to add the line: output <= input + 1;

    let mut plus_one = Communicator::<i32,i32>::new_from_serial_port("COM5").unwrap();

    let input = 10_i32;

    let output = plus_one.calculate(&input).unwrap();

    assert_eq!(input+1, output)
}

#[test]
#[allow(dead_code)]
fn rectangel_test(){

    #[derive(Vhdlizable,Debug,PartialEq)]
    struct Color{
        r: u8,
        g: u8,
        b: u8
    }
    impl Color{
        pub fn new(r: u8,g: u8, b: u8) -> Self{
            Color{r,g,b}
        }
    }

    #[derive(Vhdlizable,Debug,PartialEq)]
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

    #[derive(Vhdlizable,Debug,PartialEq)]
    struct Rectangle{
        p1: Point,
        p2: Point,
        color: Color
    }

    let rectangle = Rectangle{
        p1: Point::new(-245235245,-3),
        p2: Point::new(10,3223223),
        color: Color::new(0,32,255)
    };


    // Communicator::<Rectangle,Rectangle>::generate_vhdl_code().unwrap();
    // return;

    let mut clone_rectangle = Communicator::<Rectangle,Rectangle>::new_from_serial_port("COM5").unwrap();

    let new_rectangle = clone_rectangle.calculate(&rectangle).unwrap();

    assert_eq!(new_rectangle,rectangle);

}




