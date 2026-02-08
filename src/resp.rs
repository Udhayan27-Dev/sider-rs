use crate::resp_result::{RESPError, RESPResult};

fn binary_extract_line(buffer: &[u8],index: &mut usize)-> RESPResult<Vec<u8>>{
    let mut output = Vec::new();
    if *index >= buffer.len(){
        return Err(RespError::OutOfBounds(*index));
    }
    if buffer.len() - *index -1<2{
        *index = buffer.len();
        return Err(RESPError::OutOfBounds(*index));
    }
    
    let mut perv_elem :u8=buffer[*index].clone();
    let mut sepr_found: bool = false;
    let mut final_index: usize = *index;
    
    for &elem in buffer[*index..].iter(){
        final_index += 1;
        
        if elem == b'\n' && prev_elem ==b'\n'{
            sper_found =true;
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