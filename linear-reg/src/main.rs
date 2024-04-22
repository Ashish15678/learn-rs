pub mod matrix;
pub mod csv;
pub mod test;

use std::io;
use matrix::*;


fn main() {

    // initiate a new matrix and then proceed to take data from user 
    // for further processes

    // let mut mat = Matrix::zero(8).unwrap();
    // println!("{:?}",mat);


    csv::readCsv();
}

