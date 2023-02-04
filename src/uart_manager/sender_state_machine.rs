use std::marker::PhantomData;
use serialport::{SerialPort};
use std::io::{Error, Read};


pub enum SendResult<'a>{
    SendSuccessful(SenderStateMachine<'a>),
    SendNonSuccessful(SenderStateMachine<'a>),
    SendCompleted(),
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
    pub fn send_data(mut self) -> SendResult<'a>{
        let mut checksum = 0 as u8;

        for i in 0..=9{

            let byte_to_send = self.get_u8(self.index + i as usize);

            let byte_to_send = match byte_to_send {
                Option::Some(b) => b,
                Option::None => break,
            };

            checksum = checksum.wrapping_add(byte_to_send);

            let result = self.port.write(&[byte_to_send]);
            match result {
                Result::Ok(_) => (),
                Result::Err(e) => return SendResult::Error(e),
            }
        }

        let result = self.port.write(&[checksum]);
        match result {
            Result::Ok(_) => (),
            Result::Err(e) => return SendResult::Error(e),
        }

        let result_bytes = 0 as u8;

        let result = self.port.read(&mut [result_bytes]);
        match result {
            Result::Ok(_) => (),
            Result::Err(e) => return SendResult::Error(e),
        }

        // datas delivered correctly
        if result_bytes.count_ones() >= 4{

            println!("checksum was correct!");

            self.index += 8;

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

        let pick_to = self.data.len().min(pick_from+1)-1;

        let slice = &self.data[pick_from..pick_to];

        let mut result = 0 as u8;

        // convert to a u8 value
        for (exp,val) in slice.iter().enumerate(){
            result += (*val as u8)*2_i32.pow(exp as u32) as u8;
        }
        Option::Some(result)
    }
}