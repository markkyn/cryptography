use std::io;
use std::io::prelude::*;
use std::fs::File;

struct Block {
    data: Vec<u8>,
    rows: u8,
    cols: u8
}

impl Block {
    fn get(&self, x : u8, y : u8 ) -> &u8 {
        if x > self.cols || y > self.rows {
            return &0; 
        }
        
        let i : usize = (x * self.rows + y).into();

        return &self.data[i];
    }
}

#[allow(dead_code)]
const CIPHER_KEY : u128 = 0x2b_7e_15_16_28_ae_d2_a6_ab_f7_15_88_09_cf_4f_3c;

fn key_expansion( key : u128 ) {
    let mut temp;
    let mut i = 0;

    while i < nk{

    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("io/plain/lorem.txt")?;

    let mut buffer = Vec::new();
 
    f.read_to_end(&mut buffer)?; // &mut => Mutable Borrowing 

    // Creating blocks of bytes
    // AES input: 128 bits = 16 bytes
    for i in 0 .. buffer.len() / 16 {
        let start = i * 16;
        let end = (i+1) * 16;

        
        let data : Vec<u8> = Vec::from(&buffer[start .. end]);

        // create a block 
        let block = Block{
            data,
            rows: 4,
            cols: 4
        };
    }

    key_expansion()

    println!("Ok!");
    
    Ok(())
}

// Numeric Algorithm