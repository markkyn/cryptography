use crate::Block;
use crate::Key;
use crate::Sbox;
use crate::MIX_COLUMNS_MT;
use crate::algebra::get_polinomial;

pub fn cipher(input : &Block, mut roundKeys: Vec<Key>, sbox : Sbox) -> Block {

    let mut state : Block = input.clone();

    // pre-round
    state = add_round_key(&input, roundKeys.pop().expect("Couldnt pop round key"));

    // for round
    for r in 1..9 {

        let key = roundKeys.pop()
                .expect("Couldnt pop round key");

        state = sub_bytes(&state, sbox.clone());
        state = shift_rows(&state);

        state = mix_columns(&state, MIX_COLUMNS_MT);
        //state = add_round_key(&state, key);
    }

    // last round - 11
    state = sub_bytes(&state, sbox.clone());
    state = shift_rows(&state);
    state = add_round_key(&state, roundKeys.pop().expect("Couldnt pop round key"));

    state.clone()
}

fn add_round_key(input : &Block, key : Key) -> Block {
    let mut data : [u8; 16] = [0; 16];

    /*
        Input/State is the entry with 128 bytes (BlockSize),
    */

    for i in 0..16 { // TODO: Verify end of loop
        // Key Size: [u8; 16] = 128 bits of key
        // Block Size: [u8; 16] = 128 bits of data

        // 1:1 Xor
        data[i] = input.data[i] ^ key.key.to_be_bytes()[i];
    }

    // returns the State Block = Input XOR Key
    println!("\tAdded RoundKey: {:#02x}", input.data_as_u128());
    
    Block {
        data: data.to_vec(),
        rows: 4,
        cols: 4
    }
    
}

fn sub_bytes(input: &Block, sbox : Sbox) -> Block {

    let mut state : Block = input.clone();

    for (i, byte) in input.data.clone().into_iter().enumerate() {
        state.data[i] = sbox.get(byte);
    }
    
    println!("\tSubstitution Block: {:#02x}", state.data_as_u128());

    state.clone()
}

fn shift_rows(input: &Block) -> Block {
    let mut state : Block = input.clone();
    
    let mut rows : Vec<Vec<u8>> = state.get_rows();
    
    for r in 0..state.rows {
        rows[r as usize][(state.cols - 1) as usize] = rows[r as usize][0];

        for c in 0..(state.cols-1) {
            rows[r as usize][c as usize] = rows[r as usize][c as usize];
        }
    }
    
    println!("\tShifted Block: {:#02x}", )
    state.clone()
}

fn mix_columns(input: &Block, matrix : [u8; 16] ) -> Block {
    
    let mut state = input.clone();

    let state_cols = state.get_cols();

    let mut result_col = [0 as u32; 16];
    for c  in 0..state.cols as usize {
        for r in 0..state.rows as usize {

            let expoent_pol = get_polinomial(matrix[8 * c + r]); // from const matrix
            let value_pol = get_polinomial(state_cols[c][r]);

            let added_expoents : Vec<usize> = vec![0 as usize; 0];
            // Applying Polinomial Functions
            for i in 0..state_cols[c].len() as u8 {
                let result = (value_pol[i] + expoent_pol[i]) as usize;
                
                /* 
                // TODO: Still didn`t get this part...
                if result % 2 == 0 { // Even result are not used
                continue;
                }
                */

                added_expoents.push(result);

            }
            // m(x) = 0b100011011 (fixed expression)
            const MX : u16 = 0b100011011; 

                



            // TODO: push to vector with the result
        }
    }

    state
}
