use crate::Block;

pub fn cipher(input : &Block, roundKeys: Vec<Key>) -> Block {

    let mut state : &Block = input.clone();

    // pre-round
    state = add_round_key(&input, key.pop());

    // for round
    for r in 1..9 {
        //state = sub_bytes(state);
        //state = shift_rows(state);
        //state = mix_columns(state);
        state = add_round_key(state, key.pop());
    }

    // last round
    //state = sub_bytes(state);
    //state = shift_rows(state);
    state = add_round_key(state, kay.pop());


    return state;
}

fn add_round_key(input : &Block, key : Key) -> Block {
    let mut data : [u8; 128] = [0; 128];

    for i in 0..=128 {
        data[i] = input.data[i] ^ key.key[i];
    }

    let output : Block = {
        data: data,
        rows: 4,
        cols: 4
    };

    return output;
}