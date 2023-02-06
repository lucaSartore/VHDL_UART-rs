use std::io::{Error, ErrorKind, Read, Write};
use std::io::ErrorKind::TimedOut;
use std::ops::BitAndAssign;
use std::time::{Duration, Instant};
use serialport::SerialPort;

pub enum ReceiveResult<'a>{
    /// the message has been received successfully
    ReceiveSuccessful(ReceiveStateMachine<'a>),
    /// the message has not been received successfully
    ReceiveNonSuccessful(ReceiveStateMachine<'a>),
    /// the message has been received successfully, and we reached the end of the data to send
    ReceiveCompleted(),
    /// an error has occurred
    Error(Error)
}


/// a state machine that allows you to receive datas safely
pub struct ReceiveStateMachine<'a>{
    index: usize,
    data: &'a mut Vec<bool>,
    port: &'a mut Box<dyn SerialPort>,

}


impl<'a>  ReceiveStateMachine<'a> {
    /// create a new state medicines
    pub fn new(data: &'a mut Vec<bool>, port: &'a mut Box<dyn SerialPort>) -> Self{
        Self{
            data,
            port,
            index: 0
        }
    }

    /// try ro receive some data
    pub fn receive(mut self) -> ReceiveResult<'a>{

        // calculate how many byres i have to recive
        let mut remaining_bytes = ((self.data.len()-self.index) as f32 / 8.).ceil() as usize;

        // calculate how many byres i have to receive
        let mut byte_to_receive = 9.min(remaining_bytes);

        println!("byte_to_receive: {}",byte_to_receive);

        let in_time = Instant::now();
        let time_out = Duration::from_micros(super::WAITING_TO_RECEIVE_TIMEOUT_US);
        // i wait until all the data are in the port
        while match self.get_bytes_to_read() {
            Ok(r) => r,
            Err(e) => return ReceiveResult::Error(e)
        } < byte_to_receive as u32 +1{



            if Instant::now() - in_time > time_out{
                println!("byres_to_read was: {:?}",self.get_bytes_to_read());
                return ReceiveResult::Error(Error::new(TimedOut,"the board wasn't sending the data"))
            }

        }

        // +1 because of checksum
        let mut input_bytes = vec![0;byte_to_receive+1];

        let r = self.port.read_exact(input_bytes.as_mut_slice());

        if let Err(e) = r{
            return ReceiveResult::Error(e);
        }

        let checksum = input_bytes.remove(byte_to_receive);

        println!("DATA: {:?}, checksum: {}",input_bytes,checksum);

        // check the checksum
        if Self::is_checksum_correct(&input_bytes, checksum) == false{
            println!("the checksum was not correct!");

            let r = self.port.write(&[0]);
            if let Err(e) = r{
                return ReceiveResult::Error(e);
            }

            ReceiveResult::ReceiveNonSuccessful(
                ReceiveStateMachine{
                    index: self.index,
                    port: self.port,
                    data: self.data
                }
            )

        }else{
            println!("the checksum was correct!");

            let r = self.port.write(&[0xfc]);
            if let Err(e) = r{
                return ReceiveResult::Error(e);
            }

            self.insert_value(&input_bytes);

            if self.index == self.data.len(){
                ReceiveResult::ReceiveCompleted()
            }else{
                ReceiveResult::ReceiveSuccessful(
                    ReceiveStateMachine{
                        index: self.index,
                        port: self.port,
                        data: self.data
                    }
                )
            }

        }

    }

    fn is_checksum_correct(data: &Vec<u8>, checksum: u8) -> bool{

        let mut checksum2:u8  = 0;

        for e in data.iter(){
            checksum2 = checksum2.wrapping_add(*e)
        }

        checksum2 == checksum
    }

    fn get_bytes_to_read(&mut self) -> Result<u32,Error>{

        match self.port.bytes_to_read() {
            Result::Ok(btr) => Result::Ok(btr),
            Result::Err(e) => Result::Err(Error::new(ErrorKind::Other, format!("{e:?}")))
        }

    }

    fn insert_value(&mut self, new_value: &Vec<u8>){
        for v in new_value{

            println!("received: {v}");

            if self.index<self.data.len(){
                self.insert_byte(*v)
            }
        }
    }

    fn insert_byte(&mut self, new_value: u8){

        let mut mask: u8 = 1;

        let len = self.data.len();

        let mut c = 0;
        while self.index < len  {
            c+=1;
            // insert one bit
            self.data[self.index] = mask & new_value != 0;

            //println!("{})insert at index {} value {}",c,self.index,mask & new_value != 0);

            //shift the mask
            mask <<= 1;

            self.index+=1;

            if mask == 0{
                break
            }
        }

    }
    
}