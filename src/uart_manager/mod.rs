use std::io::{Read, Write};
use std::marker::PhantomData;
use std::{thread, time};
use std::time::Duration;
use rand::Rng;
use serialport::{Error, SerialPort, StopBits};
use crate::uart_manager::sender_state_machine::{SenderStateMachine, SendResult};
use crate::uart_manager::receiver_state_machine::{ReceiveStateMachine,ReceiveResult};

mod sender_state_machine;
mod receiver_state_machine;

/// the baud rate of the sender
const BAUD_RATE: u32 = 9600;
/// the time between a byte and the other in microseconds
const TIME_BETWEEN_BYTES_US: usize = 300;
/// the maximum number of failed sent data after witch the program will stop
const MAX_ALLOWED_FAILS: u32 = 5;
/// the timeout of the serial port in milliseconds
const TIMEOUT_US: u64 = 300;
/// the time between a byte and an other to let the state machine do his job
const TIME_BETWEEN_BYTE_US: u64 = 300;
/// the time the state machines wait for a reply after witch a time out get called
const WAITING_TO_RECEIVE_TIMEOUT_US: u64= 100_000;


pub struct UartManager{

    port: Box<dyn SerialPort>

}

impl UartManager{
    /// open the connection with a device at the specified port
    ///
    /// return the result of the item
    ///
    /// examples:
    ///  - UartManager::new("/dev/ttyUSB0")
    ///  - UartManager::new("COM5")
    pub fn new(serial_port: &str) -> Result<Self,Error>{

        let port = serialport::new(serial_port, BAUD_RATE);
        let port = port.timeout(Duration::from_micros(TIMEOUT_US));

        let port = match port.open() {
            Result::Ok(p) => p,
            Result::Err(e) => return Result::Err(e)
        };

        Result::Ok(
            Self{
                port
            }
        )

    }

    /// open the connection with a device at the specified port
    ///
    /// return item itself, and panic if the connection fails
    ///
    /// examples:
    ///  - UartManager::new("/dev/ttyUSB0")
    ///  - UartManager::new("COM5")
    pub fn new_or_panic(serial_port: &str) -> Self{
        Self::new(serial_port).unwrap()
    }


    /// send to the board a set of data,
    pub fn send_data(&mut self, data: &Vec<bool>) -> Result<(),()>{

        use sender_state_machine::{SenderStateMachine,SendResult};

        let mut sender = SenderStateMachine::new(data, &mut self.port);

        let mut fail_counter: u32 = 0;

        loop {
            sender = match sender.send_data() {
                SendResult::SendSuccessful(sender) => {
                    println!("SendSuccessful");
                    fail_counter = 0;
                    sender
                },
                SendResult::SendNonSuccessful(sender) => {
                    println!("SendNonSuccessful");
                    fail_counter += 1;
                    if fail_counter >= MAX_ALLOWED_FAILS{
                        return Result::Err(());
                    }
                    sender
                }
                SendResult::SendCompleted() => {
                    println!("SendCompleted");
                    break
                }
                SendResult::Error(e) => {
                    println!("Error: {:?}",e);
                    return Result::Err(());
                }
            };
            thread::sleep(Duration::from_millis(TIME_BETWEEN_BYTE_US));
        }
        return Result::Ok(());
    }


    /// read the data to fill the vector
    pub fn receive_data(&mut self, data: &mut Vec<bool>) -> Result<(),()>{

        let mut sm = ReceiveStateMachine::new(data, &mut self.port);

        let mut count_error = 0;

        loop {
            sm = match sm.receive() {
                ReceiveResult::ReceiveNonSuccessful(sm) => {
                    count_error += 1;
                    if count_error > MAX_ALLOWED_FAILS{
                        println!("too many consecutive errors has occur!");
                        return  Err(());
                    }
                    sm
                }
                ReceiveResult::ReceiveSuccessful(sm) => {
                    count_error = 0;
                    sm
                }
                ReceiveResult::ReceiveCompleted() => {
                    return Result::Ok(());
                }
                ReceiveResult::Error(e) => {
                    println!("Got error: {:?}",e);
                    return  Err(());
                }


            }
        }

    }
}


#[test]
fn test_round_trip(){

    const BIT_TO_SEND: usize = 91;

    fn get_rand() -> bool{
        let random_bool: bool = rand::thread_rng().gen();
        random_bool
    }

    let mut data_in = Vec::new();
    let mut data_out = vec![false;BIT_TO_SEND];


    for i in 0..BIT_TO_SEND{
        data_in.push(get_rand());
        //data_in.push(i%2 == 0);
    }

    println!("DATA_IN: {:?}",data_in);

    let mut manager = UartManager::new("COM5").unwrap();

    let _ = manager.send_data(&data_in).unwrap();

    let _ = manager.receive_data(&mut data_out).unwrap();


    println!("DATA_IN: {:?}",data_in);
    println!("DATA_OUT: {:?}",data_out);
    assert_eq!(data_out,data_in);

}

#[test]
fn test_loop(){

    for i in 0..1000{
        test_round_trip();
        thread::sleep(Duration::from_millis(10));
    }

}


#[test]
fn test_sender(){

    let mut manager = UartManager::new("COM5").unwrap();
    let r = manager.send_data(&vec![false,false,false,true,true,true]);


}

#[test]
fn test_with_on_board_checksum(){

    use rand::Rng;

    fn get_u8(pick_from: usize, data: &Vec<bool>) -> Option<u8>{

        if pick_from >= data.len(){
            return Option::None
        }

        let pick_to = data.len().min(pick_from+8)-1;

        let slice = &data[pick_from..pick_to];

        let mut result = 0 as u8;

        // convert to a u8 value
        for (exp,val) in slice.iter().enumerate(){
            result += (*val as u8)*2_i32.pow(exp as u32) as u8;
        }
        Option::Some(result)
    }

    fn get_checksum(data: &Vec<bool>) -> u8{
        let mut cs = 0 as u8;

        for i in 0..(data.len()/8+1){
            if let Some(c) = get_u8(i*8,data){
                cs = cs.wrapping_add(c);
            }
        }

        cs
    }

    fn get_rand() -> bool{
        let random_bool: bool = rand::thread_rng().gen();
        random_bool
    }

    let mut manager = UartManager::new("COM5").unwrap();

    let mut data = Vec::<bool>::new();

    for i in 0..72{
        data.push(get_rand())
    }

    let cs = get_checksum(&data);
    println!("check sum: {:b}",cs);

    let r = manager.send_data(&data).unwrap();

    println!("r: {:?}",r);

    while manager.port.bytes_to_read().unwrap() != 10{

    }

    let btr =manager.port.bytes_to_read().unwrap() as usize;

    println!("to read: {btr}");

    let mut serial_buf: Vec<u8> = vec![0; btr];
    let result = manager.port.read_exact(serial_buf.as_mut_slice());
    println!("Result: {serial_buf:?}");

    println!("result {:?}",result);

    assert_eq!(serial_buf[btr-1],cs)

}


#[test]
fn test_read(){
    use std::{thread,time};

    let port = serialport::new("COM5", BAUD_RATE);
    let port = port.timeout(Duration::from_millis(TIMEOUT_US));
    let mut port = port.open().unwrap();

    println!("started");

    let time = time::Duration::from_millis(10);
    thread::sleep(time);


    for _ in 0..1000{

        println!("{:?}",port.bytes_to_read());

        if port.bytes_to_read().unwrap() != 0{
            let mut s = String::new();

            let mut serial_buf: Vec<u8> = vec![0; 10];

            // read the result
            let result = port.read_exact(serial_buf.as_mut_slice());

            println!("Result: {serial_buf:?}");

            break
        }


        thread::sleep(time);
    }



    // let mut buffer = [0; 10];
    //
    // let mut buffer = [0; 100];
    //
    // // read up to 10 bytes
    // let r = port.read(&mut buffer);

    // let mut v:Vec<u8> = Vec::new();
    // let r = port.read_to_end(&mut v);
    // println!("resul: {:?}, vect: {:?}",r,v)
}

#[test]
fn test_manual(){

    use std::{thread,time};

    let time = time::Duration::from_micros(300);


    let port = serialport::new("COM5", BAUD_RATE);
    let port = port.timeout(Duration::from_millis(TIMEOUT_US));
    let port = port.stop_bits(StopBits::Two);
    let mut port = port.open().unwrap();



    println!("started");

    for i in 0..9{
        println!("{:?}",port.write(&[i*2]));
        thread::sleep(time);
    }


    println!("{:?}",port.write(&[72]));
    thread::sleep(time);



    let mut v = vec![0,1];
    // read the result
    println!("{:?}", port.read_to_end(&mut v));

    println!("Result: {v:?}");
}