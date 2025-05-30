use std::io;
use std::io::prelude::*;
use std::fs::File;
use crate::key::Key;
use crate::sbox_parser::Sbox;
use crate::cipher::cipher;
use crate::cipher::mix_columns;
use crate::algebra::*;

mod key;
mod sbox_parser;
mod cipher;
mod algebra;

const CIPHER_KEY : u128 = 0x2b_7e_15_16_28_ae_d2_a6_ab_f7_15_88_99_cf_4f_3c; // 128 bits = 16 bytes
const KEY_LENGTH : u8 = 4;  // Nr = 128 bits / 4 = word of 32 bits

const SBOX_BYTES : [u8; 256] = [
//   0     1     2     3      4     5     6     7  = least
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, // 0 = most
    0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0,
    0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc,
    0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a,
    0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0,
    0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b,
    0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85,
    0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
    0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17,
    0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88,
    0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c,
    0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9,
    0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6,
    0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e,
    0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94,
    0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68,
    0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

static MIX_COLUMNS_MT : [u8; 16] = [
    0x2, 0x3, 0x1, 0x1,
    0x1, 0x2, 0x3, 0x1,
    0x1, 0x1, 0x2, 0x3,
    0x3, 0x1, 0x1, 0x2
];

const MIX_COLUMNS_ROWS : usize = 4;
const MIX_COLUMNS_COLS : usize = 4;

#[derive(Clone)]
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

    fn get_i(&self, i : usize) -> &u8 {
        if i >= (self.rows * self.cols).into() {
            return &0;
        }

        &self.data[i]
    }

    fn get_rows(&self) -> Vec<Vec<u8>> {

        let mut rows = vec![vec![0u8; self.rows as usize]; self.cols as usize];

        for r in 0..self.rows {
            for c in 0..self.cols {
                rows[r as usize][c as usize] = self.data[(c + r * self.rows) as usize];
            }
        }

        rows
    }

    fn get_cols(&self) -> Vec<Vec<u8>> {
        let mut cols = vec![vec![0u8;self.cols as usize];self.rows as usize];
    
        for c in 0..self.cols {
            for r in 0..self.rows {
                cols[c as usize][r as usize] = self.data[(r + c * self.rows) as usize];
            }
        }

        cols 
    }

    fn from_rows(&mut self, rows : Vec<Vec<u8>>) -> Result<(), Vec<u8>> {
        let mut i : usize = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.data[i] = rows[r as usize][c as usize];

                i += 1;
            }
        }
        
        Ok(())
    }

    fn data_as_u128(&self) -> u128 {
        let mut integer : u128 = 0;
        
        for i in 0..16 {
            integer |= (self.data[15-i] as u128) << (8 * i);
        }

        integer
    }

}
// States are considered Blocks that are being changed over the AES.
// So, I used "type aliases" to rename the struct
// This way will be easier to understand and write the code.
type State = Block;

fn main() -> io::Result<()> {
    
    let sbox = Sbox::new(SBOX_BYTES); 
    
    // Key Expansion - Create 12 additional keys ( called round keys ) 
    // to be applied to the AES algorithm every round ( 11 rounds for 128 keys)
    let key = Key {
        key: CIPHER_KEY,
        word_count: 4 
    };

    let round_keys : Vec<Key>  = key::key_expansion(key, 11, 11, sbox.clone());

    // input
    let mut f = File::open("io/plain/lorem.txt")?;

    let mut buffer = Vec::new();
 
    f.read_to_end(&mut buffer)?; // &mut => Mutable Borrowing 

    let mut all_blocks : Vec<Block> = Vec::new();
    let mut output_blocks: Vec<Block> = Vec::new();

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
        
        println!("\nCipher Block {}: {:#02x}", i, block.data_as_u128());
        
        output_blocks.push(cipher(&block, round_keys.clone(), sbox.clone()));
    }

    println!("Ok!");
    
    Ok(())
}
