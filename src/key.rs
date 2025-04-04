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

pub fn key_expansion( key : Key , nk : usize, n_rounds : usize) {

    
    let mut round_keys : Vec<Key>;
    
    // for round
    for round in 0 .. n_rounds {
        let mut words: [u32; 4] = key.to_vec_of_word();
        
        // Operation over the last word
        rot_word(&mut words[3], 1);
        sub_word(&mut words[3]);
        r_con(&mut words[3]);

        // XOR operation over all the words\
        for ( i, word) in words.iter().enumerate().skip(1) {
            
            let key = Key {
                key = ,
                word_count: 4
            };

            


        }
    
    }


    ()
}

fn rot_word(word: &mut u32, t: usize ) { 
    // TODO: rot_word is rotating backwars, it will do for now =)
  
    let bytes = word.to_ne_bytes();

    let mut temp: [u8; 4] = [0; 4];  
    /*  t = 1 (eg.)
        
        [2b,7e,15,16] = bytes
    
        [16,2b,7e,15] = temp
    */

    // -->
    println!("word {:#02x}", word);

    for (i, _) in bytes.iter().enumerate().skip(t) {
        temp[i] = bytes[i-t];
    }
    
    // <--
    for (i, _) in bytes.iter().enumerate().take(t) {
        // +1 bc the end of the slice is exclusive
        temp[i] = bytes[bytes.len() - i - t];
    }

    *word = u32::from_ne_bytes(temp);
    
}

fn sub_word(word: &u32) {
    ()
}

fn r_con(word: &u32) {
    ()
}