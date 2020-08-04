use std::io::prelude::*;
use std::io::Result;

use super::{
    image::Image,
    binary_helper
};
pub const LENGTH_OF_LENGTH_MARKER: usize = 2; //yyyeeeeee 

pub const MARKERS: [Marker; 4] = [
    Marker {kind: MarkerType::Start, has_length: false, func: Marker::start},
    Marker {kind: MarkerType::Comment, has_length: true, func: Marker::comment},
    Marker {kind: MarkerType::DQT, has_length: true, func: Marker::dqt},
    Marker {kind: MarkerType::SOF0, has_length: true, func: Marker::sof0}
];

#[derive(Copy, Clone)]
pub struct Marker {
    pub kind: MarkerType,
    pub has_length: bool,
    pub func: fn(&mut Image, usize) -> Result<()>
}

#[derive(Debug, Copy, Clone)]
pub enum MarkerType {
    Start   = 0xD8,
    Comment = 0xFE,
    DQT     = 0xDB,
    SOF0    = 0xC0,
    _DHT,
}

impl Marker {
    fn start(_img: &mut Image, _length: usize) -> Result<()> {Ok(())}
    
    fn comment(img: &mut Image, length: usize) -> Result<()> {
        let mut comment = vec![0u8; length]; 
        img.file.read(&mut comment).unwrap();
        print!("Comment: ");
        for byte in comment {
            print!("{}", byte as char);
        }
        println!();
        Ok(())
    }

    fn dqt(img: &mut Image, length: usize) -> Result<()> {
        let (length_of_table, id_table) = binary_helper::get_tuple_as_byte(img.get_byte());
        println!("Length: {}", length);
        println!("Length of table: {}byte.", if length_of_table == 0 {1} else {2});
        println!("Id of table: {}", id_table);
        
        let mut data = vec![0u8; length - 1];
        println!("{}", img.file.read(&mut data)?);

        // TODO: add saving in dqt vector!
        for (i, byte) in data.iter().enumerate() {
            print!("{:02X} ", byte);
            if ((i+1) % 16) == 0  {println!()}
        }

        Ok(())
    }

    fn sof0(img: &mut Image, _length: usize) -> Result<()> {
        let precision = img.get_byte();
        let mut buffer = vec![0u8; 2];
        
        img.file.read(&mut buffer)?;
        let height = binary_helper::vec_as_number(&buffer);
        
        img.file.read(&mut buffer)?;
        let width = binary_helper::vec_as_number(&buffer);

        let channels_amount = img.get_byte();

        println!("Precision: {}", precision);
        println!("Image size: {}x{}", height, width);
        println!("Channels amount: {}", channels_amount); 

        for _ in 0..channels_amount {
            let id = img.get_byte();
            let (hor, ver) = binary_helper::get_tuple_as_byte(img.get_byte());
            let q_table_id = img.get_byte();
            println!("{}. H={}, V={}(q_id: {})", id, hor, ver, q_table_id);
        }

        Ok(())
    }

    fn _dht(_img: &mut Image, _length: usize) -> Result<()> {

        Ok(())
    }
}

