
pub fn get_polinomial(x: u8) -> Vec<usize> {
    /*
        Returns: A Vector of indices of the bits with value 1;
        
        Docs: https://crypto.stackexchange.com/questions/2402/how-to-solve-mixcolumns/95775#95775
    */
    
    
    let mut indexes = vec![];

    for i in 0..8 {
        if ( x >> i ) & 0x01 == 0x01 {
            indexes.push(i);
        }
    }

    indexes
}


pub fn from_polinomial(pol_vec : Vec<usize>) -> u16 {

    // it returns a number bigger than 2^7, thats why we returns a u16 instead of u8.
    // Later we need to subtract by m(x) [Reduced Polinomial] to fit our result 
    // in 8 bits


    let mut result : u16 = 0;

    for (i, pol) in pol_vec.into_iter().enumerate() {
        result |= (0b1 << pol) as u16 
    }

    result
}

pub fn pol_mult_mod2(x: Vec<usize>, y: Vec<usize>) -> Vec<usize> {
    
    // Adding the polinomials
    let mut grid : Vec<Vec<u8>> = vec![vec![0; y.len()]; x.len()];
    for i in 0..x.len() as usize{
        for j in 0..y.len() as usize{
            grid[i][j] = u8::try_from(x[i] + y[j]).expect("Overflow");
        }
    }

    // Filtering the grid value to calculate the mod 2 
    let mut polinomial : Vec<usize> = vec![];
    
    { // while scope 
        let flatted_grid : Vec<u8> = grid.into_iter().flatten().collect();
        
        let mut i = 0;
        while i < flatted_grid.len(){
            let mut count = 1;
            for j in i+1..flatted_grid.len() {
                
                if flatted_grid[j] != flatted_grid[i] {
                    break;
                }
                
                count += 1;
            }
            
            // if happens once we push to the output, bc we are at mod 2
            if count % 2 != 0 { 
                polinomial.push(flatted_grid[i].into());
            }
            
            i += count;
        }
    }

    polinomial
}

pub fn pol_u9_to_u8(polinomial_value : u16) -> u8 {
    /* 
        polinomial should be an u9, but as rust do not have a syntax for that,
        and we do  not want to over complicate the project by creating a new type
        we just use the imediate greater size type of u8;
    */

    (polinomial_value ^ 0x11b) as u8
}