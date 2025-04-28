
fn get_polinomial(x: u8) -> Vec<usize> {
    /*
        Returns: A Vector of indices of the bits with value 1;
        
        Docs: https://crypto.stackexchange.com/questions/2402/how-to-solve-mixcolumns/95775#95775
    */
    
    
    let mut indexes = vec![0 as usize;8];

    for i in 0..8 {
        if ( x >> i ) & 0x01 == 0x01 {
            indexes.push(i);
        }
    }

    indexes
}


fn from_polinomial(pol_vec : Vec<usize>) -> u16 {

    // it returns a number bigger than 2^7, thats why we returns a u16 instead of u8.
    // Later we need to subtract by m(x) [Reduced Polinomial] to fit our result 
    // in 8 bits


    let mut result : u16 = 0;

    for (i, pol) in pol_vec.into_iter().enumerate() {
        result = (pol << i) as u16 
    }

    result
}
