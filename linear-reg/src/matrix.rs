use std::{ fmt::Error};


pub struct Matrix{
    // name:String,
    mat: Vec<Vec<u8>>,
}

impl Matrix{
    

    // creates a zero matrix with suitable size
    pub fn zero(size:usize)->Result<Vec<Vec<usize>>,Error>{
        let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(size);

        // Fill each inner vector with the default value
        for _ in 0..size {
            matrix.push(vec![0; size]);
        }

        return Ok(matrix)
        }
    
    pub fn newdiagnol(size:usize)->Result<Vec<Vec<usize>>,Error>{
        let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(size);

        // Fill each inner vector with the default value
        for _ in 0..size {
            matrix.push(vec![0; size]);
        }

        return Ok(matrix)
    }
}


