use super::Vhdlizable;

impl Vhdlizable for i32{
    fn get_necessary_bits() -> usize {
        32
    }

    fn get_bit_representation(&self) -> Vec<bool> {

        let mut v = Vec::new();

        let mut mask: i32 = 1;

        // just converting the number to binary
        loop {
            v.push(
                mask&*self != 0
            );

            mask <<= 1;

            if mask == 0{
                break
            }

        }
        v

    }

    fn construct_from_bits(&mut self,v: &Vec<bool>) -> Result<(),()> {
        if v.len() != 32{
            return Err(());
        };


        *self = 0;

        for (i,n) in v.iter().enumerate(){
            if *n {
                *self += 2_i32.pow(i as u32);
            }
        }

        Ok(())
    }

    fn get_vhd_construction_code(variable_name: &str, start_index: usize) -> String {
        format!("{variable_name} <= data_in({} downto {start_index});",start_index+31)
    }

    fn get_vhd_declaration_code(variable_name: &str) -> String {
        format!("signal {variable_name}: signed(31 downto 0);")
    }

    fn get_vhd_deconstruction_code(variable_name: &str, start_index: usize) -> String {
        format!("data_out({} downto {start_index}) <= std_logic_vector({variable_name});",start_index+31)
    }
}


#[test]
fn test_stringify(){

    trait DebugPus{
        fn print(&self){
        }
    }

    impl DebugPus for i32{
        fn print(&self) {
            println!("the variable: {} has value: {}",stringify!(*self),*self)
        }
    }

    let prova = 5;

    prova.print()




}