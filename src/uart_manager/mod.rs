use std::thread;
use std::time::Duration;
use serialport::SerialPort;
use std::io::{Error,ErrorKind};
use crate::uart_manager::sender_state_machine::{SenderStateMachine, SendResult};
use crate::uart_manager::receiver_state_machine::{ReceiveStateMachine,ReceiveResult};

mod sender_state_machine;
mod receiver_state_machine;

/// the baud rate of the sender
const BAUD_RATE: u32 = 9600;
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
            Result::Err(e) => return Result::Err(Error::new(ErrorKind::Other, format!("{e:?}")))
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
    pub fn send_data(&mut self, data: &Vec<bool>) -> Result<(),Error>{

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
                        return Result::Err(Error::new(ErrorKind::Other, "too many consecutive errors has occur!"));
                    }
                    sender
                }
                SendResult::SendCompleted() => {
                    println!("SendCompleted");
                    break
                }
                SendResult::Error(e) => {
                    println!("Error: {:?}",e);
                    return Result::Err(e);
                }
            };
            thread::sleep(Duration::from_millis(TIME_BETWEEN_BYTE_US));
        }
        return Result::Ok(());
    }


    /// read the data to fill the vector
    pub fn receive_data(&mut self, data: &mut Vec<bool>) -> Result<(),Error>{

        let mut sm = ReceiveStateMachine::new(data, &mut self.port);

        let mut count_error = 0;

        loop {
            sm = match sm.receive() {
                ReceiveResult::ReceiveNonSuccessful(sm) => {
                    count_error += 1;
                    if count_error > MAX_ALLOWED_FAILS{
                        return Result::Err(Error::new(ErrorKind::Other, "too many consecutive errors has occur!"));
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
                    return  Err(e);
                }


            }
        }

    }
}


#[test]
fn test_round_trip(){

    use rand;
    use rand::Rng;

    const BIT_TO_SEND: usize = 311;

    fn get_rand() -> bool{
        let random_bool: bool = rand::thread_rng().gen();
        random_bool
    }

    let mut data_in = Vec::new();
    let mut data_out = vec![false;BIT_TO_SEND];


    for _ in 0..BIT_TO_SEND{
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

    for _ in 0..1000{
        test_round_trip();
        thread::sleep(Duration::from_millis(10));
    }

}
