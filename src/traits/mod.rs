pub mod implementations;

pub mod functions;


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
    fn construct_from_bits(&mut self,data: &Vec<bool>) -> Result<(),()>;

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


