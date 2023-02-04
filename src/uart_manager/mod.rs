use std::marker::PhantomData;
use std::time::Duration;
use serialport::{Error, SerialPort};
use crate::uart_manager::sender_state_machine::{SenderStateMachine, SendResult};


mod sender_state_machine;

/// the baud rate of the sender
const BAUD_RATE: u32 = 9600;
/// the time between a byte and the other in microseconds
const TIME_BETWEEN_BYTES_US: usize = 300;
/// the maximum number of failed sent data after witch the program will stop
const MAX_ALLOWED_FAILS: u32 = 5;

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
        let port = port.timeout(Duration::MAX);

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
                    fail_counter = 0;
                    sender
                },
                SendResult::SendNonSuccessful(sender) => {
                    fail_counter += 1;
                    if fail_counter >= MAX_ALLOWED_FAILS{
                        return Result::Err(());
                    }
                    sender
                }
                SendResult::SendCompleted() => {
                    break
                }
                SendResult::Error(_) => {
                    return Result::Err(());
                }
            }
        }
        todo!()
    }
}

#[test]
fn test_sender(){

    let mut manager = UartManager::new("COM5").unwrap();

    let r = manager.send_data(&vec![false,false,false,true,true,true]);

}