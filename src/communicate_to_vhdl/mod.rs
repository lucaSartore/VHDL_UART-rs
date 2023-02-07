use std::io::Error;
use std::marker::PhantomData;
use crate::uart_manager::UartManager;

pub mod implementations;

pub mod derive_macro;

mod functions;


/// a trait for all the object that can be transformed in a vhdl object
pub trait Vhdlizable{

    /// return how many bits are necessary to send this object to the board
    fn get_necessary_bits() -> usize;

    /// return a vector of bits that represent the object
    /// the length of the vector must be equal to get_necessary_bits()
    ///
    /// note that if the vector is \[0,0,0,1,1,1]
    /// in the vhdl code the data_in will be \[0,0,0,1,1,1]
    ///
    /// so the order stays the same
    fn get_bit_representation(&self) -> Vec<bool>;

    /// reconstruct the current item with the given data
    fn construct_from_bits(data: &Vec<bool>) -> Result<Self,Error> where Self: Sized;

    /// return a string containing the VHDL code to construct the items
    ///
    /// assume you will have a `std_logic_vector` named `data_in`, start index
    /// point to the first bit of the vector that is relevant for your data
    /// so, for example:
    ///# Examples
    ///'''
    ///     let counter: u8 = 0;
    ///
    ///     counter.get_vhd_construction_code("counter",10) // should give: "counter <= unsigned(data_in(17 downto 10));"
    ///
    ///'''
    ///
    fn get_vhd_construction_code(variable_name: &str ,start_index: usize) -> String;

    /// return a string containing the VHDL code to construct the items
    /// the vidl variable MUST be a signal
    ///
    ///# Examples
    ///'''
    ///     let counter: u8 = 0;
    ///
    ///     counter.get_vhd_declaration_code("counter") // should give: "signal counter: unsigned(7 downto 0);"
    ///
    ///'''
    ///
    fn get_vhd_declaration_code(variable_name: &str ) -> String;

    /// return a string containing the VHDL code to deconstruct the items
    ///
    /// assume you will have a `std_logic_vector` named `data_out`, start index
    /// point to the first bit of the vector that is relevant for your data
    /// so, for example:
    ///# Examples
    ///'''
    ///     let counter: u8 = 0;
    ///
    ///     counter.get_vhd_construction_code("counter",10) // should give: data_out(17 downto 10) <= std_logic_vector(counter);"
    ///
    ///'''
    ///
    fn get_vhd_deconstruction_code(variable_name: &str ,start_index: usize) -> String;
}

/// a struct that manage the connection to the board and the input and output of data
pub struct Communicator<TypeIn: Vhdlizable, TypeOut: Vhdlizable>{
    uart_manger: UartManager,
    input_type: PhantomData<TypeIn>,
    output_type: PhantomData<TypeOut>,
}

impl<TypeIn: Vhdlizable, TypeOut: Vhdlizable> Communicator<TypeIn,TypeOut>{

    /// generate a new Communicator giving the serial port
    pub fn new_from_manager(uart_manger: UartManager) -> Self{
        Self{
            uart_manger,
            input_type: PhantomData::default(),
            output_type: PhantomData::default(),
        }
    }

    ///  rty to generate a new Communicator from the given port
    pub fn new_from_serial_port(serial_port: &str) -> Result<Self, Error>{

        let um = UartManager::new(serial_port)?;

        Result::Ok(Self::new_from_manager(um))
    }

    /// generate the code of the VHDL calculator
    pub fn generate_vhdl_code() -> Result<(),Error>{
        use functions::generate_vhd_communicator;
        generate_vhd_communicator(
            TypeIn::get_necessary_bits(),
            TypeOut::get_necessary_bits(),
            TypeIn::get_vhd_declaration_code("input"),
            TypeOut::get_vhd_declaration_code("output"),
            TypeIn::get_vhd_construction_code("input",0),
            TypeOut::get_vhd_deconstruction_code("output",0)
        )?;

        println!("the file has ben generated correctly!");
        println!();
        println!("you can find the other files available at: https://github.com/lucaSartore/VHDL_UART-rs/tree/main/vhdl_sources");

        Result::Ok(())
    }

    /// calculate some data on the vhdl board
    pub fn calculate(&mut self, input: TypeIn) -> Result<TypeOut,Error>{

        let data_to_send = input.get_bit_representation();

        self.uart_manger.send_data(&data_to_send)?;

        let mut data_out = vec![false;TypeOut::get_necessary_bits()];

        self.uart_manger.receive_data(&mut data_out)?;

        let out = TypeOut::construct_from_bits(&data_out)?;

        Result::Ok(out)
    }

}
