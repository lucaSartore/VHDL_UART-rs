use std::fmt::format;
use std::marker::PhantomData;
use serialport::{SerialPort};
use std::io::{Error, ErrorKind, Read};
use std::io::ErrorKind::TimedOut;
use std::time::{Duration, Instant};
use std::thread;
use crate::uart_manager::receiver_state_machine::ReceiveResult;

pub enum SendResult<'a>{
    /// the message has been received successfully
    SendSuccessful(SenderStateMachine<'a>),
    /// the message has not been received successfully
    SendNonSuccessful(SenderStateMachine<'a>),
    /// the message has been received successfully, and we reached the end of the data to send
    SendCompleted(),
    /// an error has occurred
    Error(Error)
}

pub struct SenderStateMachine<'a>{
    index: usize,
    data: &'a Vec<bool>,
    port: &'a mut Box<dyn SerialPort>,

}

impl<'a> SenderStateMachine<'a>{
    pub fn new( data: &'a Vec<bool>, port: &'a mut Box<dyn SerialPort>) ->SenderStateMachine<'a>{
        SenderStateMachine{
            index: 0,
            data,
            port,
        }
    }
}

impl <'a> SenderStateMachine<'a>{

    /// send 9 bytes, and then the checksum the return valie
    ///
    pub fn send_data(mut self) -> SendResult<'a>{

        let time_to_sleep = Duration::from_micros(super::TIME_BETWEEN_BYTE_US);

        let mut checksum = 0 as u8;

        println!("starting sending {}/{}",self.index,self.data.len());

        // snd 9 bytes
        for i in 0..9{

            let byte_to_send = self.get_u8(self.index + i*8 as usize);

            let byte_to_send = match byte_to_send {
                Option::Some(b) => b,
                // in the case we have reached the end of the array before the 9 bytes we end
                Option::None => break,
            };

            checksum = checksum.wrapping_add(byte_to_send);

            let result = self.port.write(&[byte_to_send]);
            match result {
                Result::Ok(_) => println!("send: {}",byte_to_send),
                Result::Err(e) => return SendResult::Error(e),
            }

            thread::sleep(time_to_sleep)

        }


        // send the checksum
        let result = self.port.write(&[checksum]);
        match result {
            Result::Ok(_) => println!("send checksum: {}",checksum),
            Result::Err(e) => return SendResult::Error(e),
        }

        println!("reading reply");


        // thread::sleep(time_to_sleep);
        //
        // thread::sleep(Duration::from_millis(30));
        //
        // let mut to_read = match self.port.bytes_to_read(){
        //     Result::Ok(v) => v,
        //     Result::Err(e) => return SendResult::Error(Error::new(ErrorKind::Other,format!{"{:?}",e}))
        // };
        // if to_read == 0{
        //     thread::sleep(Duration::from_millis(3));
        //     to_read = match self.port.bytes_to_read(){
        //         Result::Ok(v) => v,
        //         Result::Err(e) => return SendResult::Error(Error::new(ErrorKind::Other,format!{"{:?}",e}))
        //     };
        // }
        // if to_read == 0{
        //     return SendResult::Error(Error::new(ErrorKind::TimedOut,"ACK didn't arrive"))
        // }
        // println!("bytes_to_read: {:?}",to_read);

        let in_time = Instant::now();
        let time_out = Duration::from_micros(super::WAITING_TO_RECEIVE_TIMEOUT_US);
        // i wait until all the data are in the port
        while match self.get_bytes_to_read() {
            Ok(r) => r,
            Err(e) => return SendResult::Error(e)
        } == 0{

            if Instant::now() - in_time > time_out{
                println!("stop waiting after {:?}", Instant::now() - in_time );
                return SendResult::Error(Error::new(TimedOut,"the board wasn't sending the data"))
            }

        }

        let mut result_bytes_vec:Vec<u8> = vec![0;1];

        // read the result
        let result = self.port.read_exact(result_bytes_vec.as_mut_slice());
        match result {
            Result::Ok(_) => (),
            Result::Err(e) => {
                println!("error in reply: {:?}",e);
                return SendResult::Error(e);
            }
        }
        let result_bytes = result_bytes_vec[0];

        println!("received ACK: {}",result_bytes);

        // datas delivered correctly
        if result_bytes.count_ones() >= 3{

            println!("checksum was correct!");

            self.index += 8*9;

            if self.index >= self.data.len(){
                SendResult::SendCompleted()
            }else{
                SendResult::SendSuccessful(
                    SenderStateMachine{
                        index: self.index,
                        data: self.data,
                        port: self.port,
                    }
                )
            }
        // error in data delivery
        }else{

            println!("checksum was incorrect!");

            SendResult::SendNonSuccessful(
                SenderStateMachine{
                    index: self.index,
                    data: self.data,
                    port: self.port,
                }
            )
        }


    }

    /// return the next 8 bits to send, return none if pick_from is >= than the length of the array
    fn get_u8(&self, pick_from: usize) -> Option<u8>{

        if pick_from >= self.data.len(){
            return Option::None
        }

        let pick_to = self.data.len().min(pick_from+8);

        let slice = &self.data[pick_from..pick_to];

        let mut result = 0 as u8;

        // convert to a u8 value
        for (exp,val) in slice.iter().enumerate(){
            result += (*val as u8)*2_i32.pow(exp as u32) as u8;
        }
        Option::Some(result)
    }

    fn get_bytes_to_read(&mut self) -> Result<u32,Error>{

        match self.port.bytes_to_read() {
            Result::Ok(btr) => Result::Ok(btr),
            Result::Err(e) => Result::Err(Error::new(ErrorKind::Other, format!("{e:?}")))
        }

    }
}

/*
#[test]
fn aaaaa(){
    for i in 0..255 as u8{

        if i.count_ones() > 3{
            println!("when \"{:08b}\" => state <= ACK;",i)
        }else{
            println!("when \"{:08b}\" => state <= NACK;",i)
        }

    }
}*/