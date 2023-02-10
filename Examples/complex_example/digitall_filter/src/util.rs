use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use vhdl_uart_rs::communicate_to_vhdl::Vhdlizable;
use crate::{RES_X, RES_Y};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

#[derive(Vhdlizable,Default,Clone,Copy,Debug,PartialEq)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Vhdlizable,Default,Clone,Copy,Debug,PartialEq)]
pub struct Gray{
    gray: u8
}
impl Color{
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Self{r,g,b}
    }
}

#[derive(Debug,PartialEq)]
pub struct Image<T: Debug>{
    pub image: [[T;crate::RES_Y];crate::RES_X]
}
impl Image<Color>{
    /// transorm a vhdl math object in a Image object so that the Vhdl trati can be implemented
    pub fn new_from_image(mat: &DynamicImage) -> Option<Self>{

        let mut image = Image::default();

        if mat.width() as usize != RES_X || mat.height() as usize != RES_Y{
            return None;
        }

        for x in 0..RES_X{
            for y in 0..RES_Y{

                let pixel = mat.get_pixel(x as u32, y as u32);
                let r = pixel.0[0];
                let g = pixel.0[1];
                let b = pixel.0[2];

                image.image[x as usize][y as usize] = Color::new(
                    r,g,b
                )

            }
        }
        Option::Some(image)
    }
    pub fn to_gray_image(&self,) -> Image<Gray>{
        let mut image = Image::<Gray>::default();

        for x in 0..RES_X{
            for y in 0..RES_Y{

                let pixel = self.image[x][y];

                let max = pixel.g.max(pixel.b.max(pixel.r));

                //different conversion type
                //let max = ((pixel.g as u16 + pixel.b  as u16+ pixel.r as u16)/3) as u8;

                image.image[x][y].gray = max
            }
        }
        image
    }
}




impl Image<Gray>{
    pub fn to_image(&self) -> DynamicImage{

        let mut image = DynamicImage::new_luma8(RES_X as u32, RES_Y as u32);

        for x in 0..RES_X{
            for y in 0..RES_Y{
                let color = self.image[x][y].gray;
                let pixel = Rgba([color,color,color,0]);
                image.put_pixel(x as u32,y as u32,pixel)
            }
        }

        image
    }
}


impl<T: Default + Clone + Copy + Debug>  Default for Image<T>{
    fn default() -> Self {
        Self{
            image: [[T::default();crate::RES_Y];crate::RES_X]
        }
    }
}


// trait to vhdl
impl<T: Vhdlizable + Debug + Default + Clone + Copy> Vhdlizable for Image<T>{
    fn get_vhd_deconstruction_code(variable_name: &str, mut start_index: usize) -> String {

        let mut s = String::new();

        for x in 0..RES_X{
            for y in 0..RES_Y{
                s.push_str(
                    &T::get_vhd_deconstruction_code(
                        &format!("{variable_name}_X{x}_Y{y}")[..],
                        start_index
                    )[..]
                );
                start_index += T::get_necessary_bits();
            }
        }
        s
    }
    fn get_vhd_declaration_code(variable_name: &str) -> String {
        let mut s = String::new();

        for x in 0..RES_X{
            for y in 0..RES_Y{
                s.push_str(
                    &T::get_vhd_declaration_code(
                        &format!("{variable_name}_X{x}_Y{y}")[..],
                    )[..]
                );
            }
        }
        s
    }
    fn get_vhd_construction_code(variable_name: &str, mut start_index: usize) -> String {

        let mut s = String::new();

        for x in 0..RES_X{
            for y in 0..RES_Y{
                s.push_str(
                    &T::get_vhd_construction_code(
                        &format!("{variable_name}_X{x}_Y{y}")[..],
                        start_index
                    )[..]
                );
                start_index += T::get_necessary_bits();
            }
        }
        s
    }
    fn get_necessary_bits() -> usize {
        RES_Y*RES_Y*T::get_necessary_bits()
    }
    fn get_bit_representation(&self) -> Vec<bool> {
        let mut v = Vec::new();
        for x in 0..RES_X{
            for y in 0..RES_Y{
                v.append(
                    &mut self.image[x][y].get_bit_representation()
                )
            }
        }
        v
    }
    fn construct_from_bits(data: &[bool]) -> Result<Self, Error> where Self: Sized {
        if data.len()   != Self::get_necessary_bits(){
            return Result::Err(Error::new(ErrorKind::Other, "the lenghr of the input was wrong"))
        }

        let mut image = Image::default();

        let mut pointer: usize = 0;

        for x in 0..RES_X{
            for y in 0..RES_Y{

                image.image[x][y] = T::construct_from_bits(
                    &data[pointer..pointer+T::get_necessary_bits()]
                )?;
                pointer += T::get_necessary_bits()
            }
        }
        Result::Ok(image)
    }
}

