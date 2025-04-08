use crate::sbox_parser::Sbox;

#[derive(Clone)]
pub struct Key {
    pub key: u128,
    pub word_count: u8, // count of words always 4
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

pub fn key_expansion( key : Key , nk : usize, n_rounds : usize, sbox : Sbox) -> Vec<Key> {
    
    let mut round_keys : Vec<Key> = Vec::new();

    round_keys.push(key.clone());

    // for round
    for round in 1 .. n_rounds {
        
        let mut words: [Word; 4] = key.to_vec_of_word();

        // Operation over the last word
        let last : usize = words.len() - 1;

        rot_word(&mut words[last], 1);
        sub_word(&mut words[last], &sbox);
        r_con(&mut words[last], round.try_into().unwrap());

        // after rcon
        let modified_word : Word = words[last].clone(); 

        words[1] ^= modified_word;
 
        for i in 1..words.len() {
            words[i] ^= words[i-1];
        }

        // to convert a a key = 4 words to a single key = u128
        //  we need [u8; 16], but we have [u32; 4]
        let round_key : u128 =
            ((words[0] as u128) << 96 ) |
            ((words[1] as u128) << 64) |
            ((words[2] as u128) << 32) |
            ((words[3] as u128));

        println!("Round key {}: {:#02x}", round, round_key);

        round_keys.push(
            Key{
                key: round_key, // need [u8; 16]
                word_count: 4
            }
        );

    }

    return round_keys; // return nothing
}

fn rot_word(word: &mut Word, t: usize ) { 
    let bytes = word.to_be_bytes();

    let mut temp: [Byte; 4] = [0; 4];  
    /*  t = 1 (eg.)
        
        [99, cf, 4f, 3c] = bytes
    
        [cf, 4f, 3c, 99] = temp
    */
    
    // <--
    for i in 0 .. bytes.len() - t {
        temp[i] = bytes[i + t];
    }

    // -->
    for i in 0 .. t {
        temp[bytes.len()-1] = bytes[i];
    }

    *word = u32::from_be_bytes(temp);
}

fn sub_word(word: &mut Word, sbox: &Sbox) {
    let mut temp : [ Byte; 4 ] = [0;4];

    for(i, byte) in word.to_be_bytes().iter().enumerate() {
        temp[i] = sbox.get(*byte);
    }

    *word = u32::from_be_bytes(temp);
}

fn r_con(word: &mut Word, i_round: usize){
    const RCON : [u8; 10] = [
        0x01, 0x02, 0x04, 0x08, 0x10,
        0x20, 0x40, 0x80, 0x1B, 0x36 
    ];

    let mut bytes = word.to_be_bytes();

    bytes[0] ^= RCON[i_round];

    
    *word = u32::from_be_bytes(bytes);

}
