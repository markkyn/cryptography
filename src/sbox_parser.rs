pub struct Sbox {
    data: Vec<u8>
}

impl Sbox {
    pub fn get(&self, byte : u8 ) -> u8 {
        /*
            byte 0x9F = 9 * 2¹ + F * 2°;
                        ^        ^
                       most      least
        */
        
        let most_sig  = byte >> 4;   // eg. 9
        let least_sig = byte & 0b1111; // eg. F

        
        let i : usize = (most_sig * 16 + least_sig).into();

        self.data[i]
    }

    pub fn new(sub_bytes : [u8; 256]) -> Sbox{
        Sbox {
            data : Vec::from(&sub_bytes[..]),
        }
    }
}
