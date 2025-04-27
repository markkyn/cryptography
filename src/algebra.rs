fn get_polinomial(x: u8) -> Vec<usize> {
    /*
        Returns: A Vector of indices of the bits with value 1;
        
        Docs: https://crypto.stackexchange.com/questions/2402/how-to-solve-mixcolumns/95775#95775
    */
    
    
    let mut indexes = vec![usize;8]

    for i in 0..8 {
        if ( x >> i ) & 0x01 == 0x01 {
            indexes.push(i);
        }
    }

    indexes
}