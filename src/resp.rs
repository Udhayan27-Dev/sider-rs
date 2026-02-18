use crate::resp_result::{RESPError, RESPResult};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RESP {
    SimpleString(String),
    BulkString(String),
    Null,
}

impl fmt::Display for RESP{
    fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result{
        let data = match self{
            Self::SimpleString(data) => format!("+{}\r\n",data),
            Self::BulkString(data) => format!("${}\r\n{}\r\n",data.len(),data),
            Self::Null => String::from("$-1\r\n"),
        };
        write!(f,"{data}")
    }
}

//6.This func remove the structure of the RESP and send the raw 
//data bytes to the as_string func...this excludes the \r\n
pub fn binary_extract_line(buffer: &[u8], index: &mut usize) -> RESPResult<Vec<u8>> {
    let mut output = Vec::new();

    //prevents reading the value in the end of buffer
    if *index >= buffer.len() {
        return Err(RESPError::OutOfBounds(*index));
    }

    //checks for the space for the \r\n
    if buffer.len() - *index - 1 < 2 {
        *index = buffer.len();
        return Err(RESPError::OutOfBounds(*index));
    }

    let mut prev_elem: u8 = buffer[*index].clone();
    let mut sepr_found: bool = false;
    let mut final_index: usize = *index;

    for &elem in buffer[*index..].iter() {
        final_index += 1;

        if elem == b'\n' && prev_elem == b'\r' {
            sepr_found = true;
            break;
        }
        prev_elem = elem.clone();
    }

    if !sepr_found {
        *index = final_index;
        return Err(RESPError::OutOfBounds(*index));
    }
    output.extend_from_slice(&buffer[*index..final_index - 2]);
    *index = final_index;
    Ok(output)
}

//5.This func converts the bytes to string and also handles the Error
pub fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let line = binary_extract_line(buffer, index)?;
    Ok(String::from_utf8(line)?)
}


//4.This func removes the type of the response i.e '+','$'...
pub fn resp_remove_type(value: char, buffer: &[u8], index: &mut usize) -> RESPResult<()> {
    if buffer[*index] != value as u8{
        return Err(RESPError::WrongType);
    }
    *index += 1;
    Ok(())
}

//3.if the type of the RESP,this func handles the bytes and process it to the String
fn parse_simple_string(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    resp_remove_type('+', buffer, index)?;
    let line: String = binary_extract_line_as_string(buffer, index)?;
    Ok(RESP::SimpleString(line))
}

//2.Checks the type of the RESP from the client and routes the bytes to the func that handles the type. 
fn parse_router(buffer:&[u8],index:&mut usize) -> Option<fn(&[u8],&mut usize) -> RESPResult<RESP>>{
    match buffer[*index]{
        b'+' => Some(parse_simple_string),
        _ => None,
    }
}

//1.gets the bytes from the client and sends it to the router
pub fn bytes_to_resp(buffer:&[u8],index:&mut usize) ->  RESPResult<RESP>{
    match parse_router(buffer,index){
        Some(parse_func) => {
            let result:RESP = parse_func(buffer,index)?;
            Ok(result)
        }
        None => Err(RESPError::Unknown),
    }
}

//to extract a given amount of bytes from the buffer
fn binary_extract_bytes(buffer: &[u8],index: &mut usize,length:usize) -> RESPResult<Vec<u8>>{
    let mut output = Vec::new();
    
    if *index + length > buffer.len(){
        return Err(RESPError::OutOfBounds(buffer.len()));
    }
    
    output.extend_from_slice(&buffer[*index..*index + length]);
    
    *index += length;
    Ok(output)
    
}

