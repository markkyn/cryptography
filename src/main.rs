use std::io;
use std::io::prelude::*;
use std::fs::File;
use crate::key::Key;

mod key;

const CIPHER_KEY : u128 = 0x2b_7e_15_16_28_ae_d2_a6_ab_f7_15_88_09_cf_4f_3c; // 128 bits = 16 bytes
const KEY_LENGTH : u8 = 4;  // Nr = 128 bits / 4 = word of 32 bits

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

// States are considered Blocks that are being changed over the AES.
// So, I used "type aliases" to rename the struct
// This way will be easier to understand and write the code.
type State = Block;

fn main() -> io::Result<()> {
    let mut f = File::open("io/plain/lorem.txt")?;

    let mut buffer = Vec::new();
 
    f.read_to_end(&mut buffer)?; // &mut => Mutable Borrowing 

    let mut all_blocks : Vec<Block> = Vec::new();

    let mut block : Block;

    // Creating blocks of bytes
    // AES input: 128 bits = 16 bytes
    for i in 0 .. buffer.len() / 16 {
        let start = i * 16;
        let end = (i+1) * 16;
        
        let data : Vec<u8> = Vec::from(&buffer[start .. end]);

        // create a block 
        block = Block{
            data,
            rows: 4,
            cols: 4
        };

        all_blocks.push(block);
    }

    let key = Key {
        key: CIPHER_KEY,
        word_count: 4
    };

    // Key Expansion - Create 12 additional keys ( called round keys ) 
    // to be applied to the AES algorithm every round ( 11 rounds for 128 keys)
    let _round_keys : ()  = key::key_expansion(key, 3, 3);


    println!("Ok!");
    
    Ok(())
}
