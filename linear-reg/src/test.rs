#[cfg(test)]

use crate::csv::add;
use crate::csv::sub;

#[test]
fn test_add(){
    let result = add(4,5).unwrap();
    assert_eq!(result,9);

}

#[test]
fn test_sub(){
    let result = sub(4,5).unwrap();
    assert_eq!(result,9);

}