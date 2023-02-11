# VHDL_UART-rs
a rust library that allows you to send and receive data to a VHDL program (running in this case on a nexys 4 DDR board) using a serial interface

## how to use it

the library defines the `Vhdlizable` trait, which is implemented for all integers and for bool  
you will be able to send/receive to/from the board only items that implement this trait

### Rust code

you can can create a new instance of `Communicator`, to communicate with the board

```rust
fn plus_one(x: i32) -> Result<i32,Error>{

    // if you nead the sourcecode uncomment this line
    //let _ = Communicator::<i32,i32>::generate_vhdl_code()?;

    let mut plus_one = Communicator::<i32,i32>::new_from_serial_port(USB_PORT)?;

    let result = plus_one.calculate(&x)?;

    return Ok(result);
}
```
as you can see you can use the generic type annotation to specify the type of the input and of the output  


once you have create the instance, you can call the `calculate` method to send send and receive data

-----------------------------------------------------------------

note that the trait is also derivable
```rust
    #[derive(Vhdlizable)]
    #[derive(Default,PartialEq,Debug)]
    struct Color{
        red: u8,
        green: u8,
        blue: u8,
    }

    #[derive(Vhdlizable)]
    #[derive(Default,PartialEq,Debug)]
    struct Point{
        x: i32,
        y: i32
    }

    #[derive(Vhdlizable)]
    #[derive(Default,PartialEq,Debug)]
    struct Triangle{
        point_1: Point,
        point_2: Point,
        point_3: Point,
        fill_color: Color
    }
```

### from Rust to VHDL

you can then generate the VHDL code in one of these two way:

```rust
let _ = Communicator::<i32,i32>::generate_vhdl_code();
```

```rust
let mut plus_one = Communicator::<i32,i32>::new_from_serial_port(USB_PORT).unwrap();
plus_one.generate_vhdl_code_from_instance();

```

at this point, the data structure in rust will be converted in data structure in VHDL, so for example, in the case of the Triangle the conversion will be:

```VHDL
    signal input_point_1_x: signed(31 downto 0);
    signal input_point_1_y: signed(31 downto 0);
    signal input_point_2_x: signed(31 downto 0);
    signal input_point_2_y: signed(31 downto 0);
    signal input_point_3_x: signed(31 downto 0);
    signal input_point_3_y: signed(31 downto 0);
    signal input_fill_color_red: unsigned(7 downto 0);
    signal input_fill_color_green: unsigned(7 downto 0);
    signal input_fill_color_blue: unsigned(7 downto 0);
```

note that all the names will be generated automatically using the internal name of the struct, thanks to rust macro!

### VHDL code
now all you need to do is write the remeanint VHDL code  

so for example in the case of the plus one function, the only VHDL code that is not generated automatically an has to be handwritten is:
```VHDL
    --      INSERT HERE YOUR VHDL CODE
    output <= input + 1
```
