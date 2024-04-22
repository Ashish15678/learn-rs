use std::{fs::File, io, process, result};

use csv::{Error, StringRecord};


/// ## Reads csv data using filepath and csv crate
pub fn readCsv() ->Vec<StringRecord>{
    
    //opening file 
    let file = File::open("diabets.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut mat:Vec<StringRecord> = vec![];
    for result in rdr.records(){
        mat.push(result.unwrap());
    //    println!("{:?}",result.unwrap());
    }

    println!("{:?}",mat);
    println!("{:?}",mat[0]);
    mat
}