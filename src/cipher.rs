use crate::Block;
use crate::Key;
use crate::Sbox;
use crate::MIX_COLUMNS_MT;
use crate::algebra::get_polinomial;
use crate::algebra::from_polinomial;
use crate::algebra::pol_mult_mod2;
use crate::algebra::pol_u9_to_u8;

pub fn cipher(input : &Block, mut roundKeys: Vec<Key>, sbox : Sbox) -> Block {

    let mut state : Block = input.clone();
    
    let permutate : [u8; 16] = [
        0x00, 0x05, 0x0a, 0x0f,
        0x04, 0x09, 0x0e, 0x03,
        0x08, 0x0d, 0x02, 0x07,
        0x0c, 0x01, 0x06, 0x0b
    ]; 

    // pre-round
    state = add_round_key(&input, roundKeys[0].clone());

    // for round
    for r in 1..9 {

        println!("\n\tRound {}:\n", r);

        println!("\tInput Block: {:#02x}", state.data_as_u128() );
        state = sub_bytes(&state, sbox.clone());
        //state = shift_rows(&state);
        state = permutation(&state, permutate);


        state = mix_columns(&state, MIX_COLUMNS_MT);
        state = add_round_key(&state, roundKeys[r].clone());
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

    for i in 0..16 {
        // Key Size: [u8; 16] = 128 bits of key
        // Block Size: [u8; 16] = 128 bits of data

        // 1:1 Xor
        data[i] = input.data[i] ^ key.get_byte(i).unwrap();
    }

    print!("\n");
    // returns the State Block = Input XOR Key

    
    let output = Block {
        data: data.to_vec(),
        rows: 4,
        cols: 4
    };

    println!("\tAdded RoundKey: {:#02x}", output.data_as_u128());
    
    output.clone()
}

fn sub_bytes(input: &Block, sbox : Sbox) -> Block {

    let mut state : Block = input.clone();

    for (i, byte) in input.data.clone().into_iter().enumerate() {
        state.data[i] = sbox.get(byte);
    }
    
    println!("\tSubstitution Block: {:#02x}", state.data_as_u128());

    state.clone()
}

fn permutation(input: &Block, permutate : [u8; 16]) -> Block {
    // i = permute[r]
    // output[r] = input[i]

    let mut state : Block = input.clone();

    //let mut rows : Vec<Vec<u8>> = state.get_rows();

    for r in 0..16 {
        let i : usize =  permutate[r] as usize;

        state.data[r] = input.data[i];
    }

    let output = Block {
        data: state.data,
        rows: state.rows,
        cols: state.cols
    };
    
    println!("\tShifted Block: {:#02x}", output.data_as_u128());
    output.clone()
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
    
    
    let output = Block{
        data:  rows.into_iter().flatten().collect(),
        rows: state.rows,
        cols: state.cols
    };

    println!("\tShifted Block: {:#02x}", output.data_as_u128());
    output.clone()
}

fn mix_columns(input: &Block, matrix : [u8; 16] ) -> Block {
    
    let state = input.clone();

    let mut state_cols : Vec<Vec<u8>> = state.get_cols();
    let data : u128 = 0x00;

    let mut output = state.clone();
    for c  in 0..state.cols as usize { // For each col in expoent matrix 
        for r in 0..state.rows as usize { // Calculate each polinomial ( )
            
            let mut result_byte : u8 = 0; // x¹ + x² + x³ =  element of polinomials;
            for e in 0..state.rows as usize { // for each element
                let expoent_pol = get_polinomial(matrix[4 * r + e]); // from const matrix
                let value_pol = get_polinomial(state_cols[e][c]);

                // Polinomial multiplication mod 2
                let added_expoents : Vec<usize> = pol_mult_mod2(value_pol, expoent_pol);
                
                let integer_polinomial : u16 = from_polinomial(added_expoents);
                
                let mut byte_polinomial : u8 = 0x00;
                if integer_polinomial > 0xFF {
                    byte_polinomial = pol_u9_to_u8(integer_polinomial);
                } else {
                    byte_polinomial = integer_polinomial as u8;
                }
                
                // XORing => in x¹ + x² + x³, we apply XOR for every polinomial sum in GF(2^8) 
                result_byte ^= byte_polinomial;
            }

            output.data[4 *c + r] = result_byte;
        }
    }

    println!("\tMixed Columns: {:#02x}", output.data_as_u128());

    state
}
