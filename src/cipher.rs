use crate::Block;
use crate::Key;

pub fn cipher(input : &Block, mut roundKeys: Vec<Key>) -> Block {

    let mut state : &Block = &input.clone();

    // pre-round
    state = add_round_key(&input, roundKeys.pop().expect("Couldnt pop round key"));

    // for round
    for r in 1..9 {

        let key = roundKeys.pop()
                .expect("Couldnt pop round key");

        //state = sub_bytes(state);
        //state = shift_rows(state);
        //state = mix_columns(state);
        state = add_round_key(&state, key);
    }

    // last round - 11
    //state = sub_bytes(state);
    //state = shift_rows(state);
    state = add_round_key(input, roundKeys.pop().expect("Couldnt pop round key"));

    return state.clone();
}

fn add_round_key(input : &Block, key : Key) -> Block {
    let mut data : [u8; 16] = [0; 16];

    /*
        Input/State is the entry with 128 bytes (BlockSize),


    */

    for i in 0..128 {
        // Key Size: [u8; 16] = 128 bits of key
        // Block Size: [u8; 16] = 128 bits of data

        // 1:1 Xor
        data[i] = input.data[i] ^ key.key.to_be_bytes()[i];
    }

    // returns the State Block = Input XOR Key
    Block {
        data: data.to_vec(),
        rows: 4,
        cols: 4
    };
}