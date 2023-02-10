use vhdl_uart_rs::communicate_to_vhdl::Communicator;
use util::Image;
use image::open;
use crate::util::{Color, Gray};

pub const RES_X: usize = 64;
pub const RES_Y: usize = 64;

#[allow(dead_code)]
mod util;

fn main() {

    let image_og = open("..\\test1.jpg").unwrap().resize_exact(RES_X as u32, RES_Y as u32,  image::imageops::FilterType::Nearest);


    let color_image = Image::new_from_image(&image_og).unwrap();

    //let _ = Communicator::<Image<Color>,Image<Gray>>::generate_vhdl_code().unwrap();
    //return;


    let mut to_gray = Communicator::<Image<Color>,Image<Gray>>::new_from_serial_port("COM5").unwrap();

    let gray_image = to_gray.calculate(&color_image).unwrap();

    let gray_image_control = color_image.to_gray_image();

    assert_eq!(gray_image,gray_image_control);
    println!("the 2 images where exactly equal!");

    let final_image = gray_image.to_image();

    final_image.save("..\\result.jpg").unwrap();

}

