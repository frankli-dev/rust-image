pub fn vec_as_number(vec: &Vec<u8>) -> usize {
    let mut answer: usize = vec[0] as usize;

    for &num in &vec[1..] {
        answer = (answer << 8) | (num as usize);
    }
    answer 
}

pub fn get_tuple_as_byte(byte: u8) -> (u8, u8) {
    (byte >> 4 & 0xF, byte & 0xF)
}


//TODO: Do this stuff
pub fn _vec_to_zigzag_order(vec: &Vec<u8>) -> Vec<u8> {
    let length = vec.len();
    let (mut _num, mut _i, mut _j) = (0, 0, 0);
    let new_vector = vec![0u8; length];
    while _num < length * length {
        _i += 1;
        _j += 1;
        _num += 1;
    }
    new_vector
}
