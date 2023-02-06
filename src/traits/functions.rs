use std::marker::PhantomData;
use crate::traits::Vhdlizable;

/// generate the vhdl file necessary for the board to be able to read the input
pub fn generate_vhdl_code<T: Vhdlizable, E: Vhdlizable>(path: &str, input: PhantomData<T>,output: PhantomData<E>) -> Result<(),()>{


    todo!()
}

pub fn calculate_on_fpga<T: Vhdlizable, E: Vhdlizable>(input: &T,output: &mut E) -> Result<(),()>{


    todo!()
}
