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


pub fn key_expansion( key : Key , nk : u8, nr : u8) {

    let words: [u32; 4] = key.to_vec_of_word();

    let i = 0;
    for word in words.iter() {
        rot_word(word, 1);
        sub_word(word);
        r_con(word);
        
        println!("word[{}] = {}", i, word);
    }

    ()
}

pub fn rot_word(word: &u32, t: usize ) -> u32 {
    let bytes = word.to_ne_bytes();

    let mut temp: [u8; 4] = [0; 4];  
    /*  t = 1 (eg.)
        
        [1,2,3,4] = bytes
    
        [4,1,2,3] = temp
    */


    // -->
    for (i, _) in bytes[t..].iter().enumerate() {
        temp[i] = bytes[i];
        println!("{}", bytes[i]);
    }

    // <--
    for (i, _) in bytes[..t+1].iter().enumerate() {
                              // +1 bc the end of the slice is exclusive
        temp[i] = bytes[bytes.len() - i - t + 1];
    }

    u32::from_ne_bytes(temp)
}

fn sub_word(word: &u32) {
    ()
}

fn r_con(word: &u32) {
    ()
}