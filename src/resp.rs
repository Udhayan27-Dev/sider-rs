use crate::resp_result::{RESPError, RESPResult};

#[derive(Debug,PartialEq)]
pub enum RESP{
    SimpleString(String),
}

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

pub fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let line = binary_extract_line(buffer, index)?;
    Ok(String::from_utf8(line)?)
}

pub fn resp_remove_type(value:char,buffer:&[u8],index:&mut usize)-> RESPResult<()>{
    if buffer[*index] != value as u8{
        return Err(RESPError::WrongType)
    }
    *index += 1;
    Ok(())
}

fn parse_simple_string(buffer:&[u8],index:&mut usize ) -> RESPResult<RESP>{
    resp_remove_type('+',buffer,index)?;
    let line: String = binary_extract_line_as_string(buffer, index)?;
    Ok(RESP::SimpleString(line))
}