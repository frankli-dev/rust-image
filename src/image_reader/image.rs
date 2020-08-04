use std::fs::File;
use std::io::prelude::*;
use std::io::{
    Error,
    ErrorKind,

    Result
};

use super::{
    binary_helper,
    marker
};
pub struct Image {
    pub file: File,
    pub _dqt_table: Vec<i32>
}

impl Image {
   pub fn get_marker(&mut self) -> Result<&marker::Marker> {
        let mut buffer = vec![0u8; 2];
        self.file.read(&mut buffer)?;
        if buffer[0] != 0xFF {
            return Err(Error::new(ErrorKind::Other, "Previous marker is not completely clear!"));
        }

        for marker in marker::MARKERS.iter() {
            if marker.kind as u8 == buffer[1] {
                cyan_ln!("Found {:?} marker. Execute!", marker.kind);
                let length = if marker.has_length {
                    self.file.read(&mut buffer)?;
                    binary_helper::vec_as_number(&buffer) - marker::LENGTH_OF_LENGTH_MARKER
                } else {
                    0
                };
                let func = marker.func;
                func(self, length)?;
                return Ok(marker);
            } 
        }

        return Err(Error::new(ErrorKind::Other, format!("Marker 0x{:02X} not found!", buffer[1])));
    }
}

//File helper
impl Image {
   pub fn get_byte(&mut self) -> u8 {
        let mut buffer = 0;
        self.file.read(std::slice::from_mut(&mut buffer)).unwrap();
        buffer
    }
}

