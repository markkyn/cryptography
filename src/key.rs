use crate::sbox_parser::Sbox;

pub struct Key {
    pub key: u128,
    pub word_count: u8, // count of words
}

impl Key {
    fn get(&self, n_word : u8 ) {
    
        if n_word > self.word_count {
            ( )
        }

        let _keys = self.to_vec_of_word();

        let _word_len = self.word_len();

        // returns a slice
        ( )
    }

    pub fn to_vec_of_word(&self) -> [u32; 4] {
    
        let bytes = self.key.to_be_bytes();

        let chunks = bytes.chunks(4);

        let result : [u32; 4] = chunks
            .map(|c| 
                {
                    let a : [u8; 4] = c.try_into().expect("Wrong size");
                    u32::from_be_bytes(a)
                })
            .collect::<Vec<u32>>()
            .try_into()
            .expect("Result should have exactly 4 elements");

        return result;
    }

    fn word_len(&self) -> u8 {
        // len of each word
        // eg. 128 bits / 4 = 32 bits;

        (128 / self.word_count)
            .try_into()
            .expect("A quantidade de words está fora do escopo")
    }
}


type Byte = u8;
type Word = u32;

pub fn key_expansion( key : Key , nk : usize, n_rounds : usize, sbox : Sbox) {
    
    let mut round_keys : Vec<Key>;
    
    // for round
    for round in 0 .. n_rounds {
        
        let mut words: [Word; 4] = key.to_vec_of_word();


        // Operation over the last word
        let last : usize = words.len() - 1;

        rot_word(&mut words[last], 1);
        sub_word(&mut words[last], &sbox);
        r_con(&mut words[last], round.try_into().unwrap());

        // XOR operation over all the words\
        // for word in key
        for ( i, word ) in words.iter().enumerate() {

        }
    
    }

    () // return nothing
}

fn rot_word(word: &mut Word, t: usize ) { 
    let bytes = word.to_be_bytes();

    let mut temp: [Byte; 4] = [0; 4];  
    /*  t = 1 (eg.)
        
        [99, cf, 4f, 3c] = bytes
    
        [3c, 99, cf, 4f] = temp
    */
    
    println!("Word before rotation {:#02x}", word);
    
    // -->
    for (i, _) in bytes.iter().enumerate().skip(t) {
        temp[i] = bytes[i-t];
    }
    
    // <--
    for (i, _) in bytes.iter().enumerate().take(t) {
        // +1 bc the end of the slice is exclusive
        temp[i] = bytes[bytes.len() - i - t];
    }

    *word = u32::from_be_bytes(temp);

    println!("     after rotation {:#02x}", word);
}

fn sub_word(word: &mut Word, sbox: &Sbox) {
    let mut temp : [ Byte; 4 ] = [0;4];

    println!("Word before substitution {:#02x}", word);

    for(i, byte) in word.to_be_bytes().iter().enumerate() {
        temp[i] = sbox.get(*byte);
    }

    *word = u32::from_be_bytes(temp);

    println!("     after substitution {:#02x}", word);
}

fn r_con(word: &mut Word, i_round: usize){
    const RCON : [u8; 10] = [
        0x01, 0x02, 0x04, 0x08, 0x10,
        0x20, 0x40, 0x80, 0x1B, 0x36 
    ];

    let mut bytes = word.to_be_bytes();

    println!("Rcon: {:#02x} -> {:#02x}", bytes[0], RCON[i_round]);
    bytes[0] ^= RCON[i_round];

    
    *word = u32::from_be_bytes(bytes);

    println!("\t after rcon: {:#02x}", word);

}