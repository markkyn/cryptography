use crate::Block;
use crate::Key;

pub fn cipher(input : &Block, roundKeys: Vec<Key>) -> Block {

    let mut state : Block = *input;

    // pre-round
    state = add_round_key(&input, roundKeys[0].pop());

    // for round
    for r in 1..9 {

        let key = roundKeys[r];

        //state = sub_bytes(state);
        //state = shift_rows(state);
        //state = mix_columns(state);
        state = add_round_key(&state, key.pop());
    }

    // last round
    //state = sub_bytes(state);
    //state = shift_rows(state);
    state = add_round_key(&roundKeys[0], kay.pop());


    return state;
}

fn add_round_key(input : &Block, key : Key) -> Block {
    let mut data : [u8; 16] = [0; 16];

    /*
        Input/State is the entry with 128 bytes (BlockSize),


    */

    for i in 0..128 {
        data[i] = input.data[i] ^ key.key[i];
    }

    let output : Block = Block {
        data: data,
        rows: 4,
        cols: 4
    };

    return output;
}