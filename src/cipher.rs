pub fn cipher(input : &Block, output : &Block ){

    let state = output;

    add_round_key();

    for r in 1..9 {
        sub_bytes();
        shift_rows();
        mix_columns();
        add_round_key();
    }

    sub_bytes();
    shift_rows();
    add_round_key();

    state;
}