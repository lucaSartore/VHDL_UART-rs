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

    let mut calcolate_middle_point = Communicator::<Rectangle,Point>::new_from_serial_port("COM5").unwrap();

    let middle_point = calcolate_middle_point.calculate(&rectangle).unwrap();

    //println!("the middle point of the rectange {:?} is {:?}",rectangle,middle_point)


}