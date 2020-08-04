mod image_reader;

use std::fs::File;
use image_reader::image;

#[macro_use]
extern crate colour;

fn main(){
    let mut image = image::Image {file: File::open("image.jpg").unwrap(), _dqt_table: vec![1,2]};
//    let vec: Vec<u8> = vec![0x3A, 0x24]; 
//    println!("Ans of array {:?} is {}", vec, binary_helper::vec_as_number(&vec));
//    println!("Half: {}", binary_helper::get_half_of_byte(0x11, TypeOfHalf::First));
   loop {
        match image.get_marker() {
            Ok(_) => {},
            Err(msg) => {
                println!("Error!! {}", msg);
                break;
            }
        }
    }
}

