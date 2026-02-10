use crate::resp_result::{RESPError, RESPResult};

fn binary_extract_line(buffer: &[u8],index: &mut usize)-> RESPResult<Vec<u8>>{
    let mut output = Vec::new();
    
    //prevents reading the value in the end of buffer
    if *index >= buffer.len(){
        return Err(RespError::OutOfBounds(*index));
    }
    
    //checks for the space for the \r\n
    if buffer.len() - *index -1 < 2{
        *index = buffer.len();
        return Err(RESPError::OutOfBounds(*index));
    }
    
    let mut prev_elem :u8=buffer[*index].clone();
    let mut sepr_found: bool = false;
    let mut final_index: usize = *index;
    
    for &elem in buffer[*index..].iter(){
        final_index += 1;
        
        if elem == b'\n' && prev_elem ==b'\r'{
            sepr_found =true;
            break;
        }
        prev_elem = elem.clone();
    }
    
    if !sepr_found {
        *index = final_index;
            return Err(RESPError::OutOfBounds(*index));
    }
    output.extend_from_slice(&buffer[*index..final_index-2]);
    *index = final_index;
    Ok(output)
}




#[cfg(test)]
mod tests {
use super::*;
#[test]
fn test_binary_extract_line_empty_buffer() {
let buffer = "".as_bytes();
let mut index: usize = 0;
match binary_extract_line(buffer, &mut index) {
Err(RESPError::OutOfBounds(index)) => {
assert_eq!(index, 0);
}
_ => panic!(),
}
}
#[test]
fn test_binary_extract_line_single_character() {
let buffer = "O".as_bytes();
let mut index: usize = 0;
match binary_extract_line(buffer, &mut index) {
Err(RESPError::OutOfBounds(index)) => {
assert_eq!(index, 1);
}
_ => panic!(),
}
}
#[test]
fn test_binary_extract_line_index_too_advanced() {
let buffer = "OK".as_bytes();
let mut index: usize = 1;
match binary_extract_line(buffer, &mut index) {
Err(RESPError::OutOfBounds(index)) => {
assert_eq!(index, 2);
}
_ => panic!(),
}
}
#[test]
fn test_binary_extract_line_no_separator() {
let buffer = "OK".as_bytes();
let mut index: usize = 0;
match binary_extract_line(buffer, &mut index) {
Err(RESPError::OutOfBounds(index)) => {
assert_eq!(index, 2);
}
_ => panic!(),
}
}
#[test]
fn test_binary_extract_line_half_separator() {
let buffer = "OK\r".as_bytes();
let mut index: usize = 0;
match binary_extract_line(buffer, &mut index) {
Err(RESPError::OutOfBounds(index)) => {
assert_eq!(index, 3);
}
_ => panic!(),
}
}
#[test]
fn test_binary_extract_line_incorrect_separator() {
let buffer = "OK\n".as_bytes();
let mut index: usize = 0;
match binary_extract_line(buffer, &mut index) {
Err(RESPError::OutOfBounds(index)) => {
assert_eq!(index, 3);
}
_ => panic!(),
}
}
#[test]
fn test_binary_extract_line() {
let buffer = "OK\r\n".as_bytes();
let mut index: usize = 0;
let output = binary_extract_line(buffer, &mut index).unwrap();
assert_eq!(output, "OK".as_bytes());
assert_eq!(index, 4);
}
}