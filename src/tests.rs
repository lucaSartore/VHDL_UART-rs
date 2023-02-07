use crate::communicate_to_vhdl::{Communicator,Vhdlizable};

#[test]
fn plus_one_test(){
    //to generate the VHDL code use:
    //let _ = Communicator::<i32,i32>::generate_vhdl_code();
    //and remember to add the line: output <= input + 1;

    let mut plus_one = Communicator::<i32,i32>::new_from_serial_port("COM5").unwrap();

    let input = 10_i32;

    let output = plus_one.calculate(input).unwrap();

    assert_eq!(input+1, output)
}
