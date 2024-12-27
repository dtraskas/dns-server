use super::utils;
use utils::ToBigEndian;

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub names: Vec<String>,
    pub qtype: u16,
    pub qclass: u16,
}

impl DnsQuestion {
    pub fn new(buffer: Vec<u8>, cursor: &mut usize) -> Self {
        let mut names = vec![];
        let mut pointer_cursor: usize = *cursor;
        let mut is_pointer: bool = false;
        while buffer[pointer_cursor] != 0x00 {
            // The first two bits are ones.  This allows a pointer to be distinguished
            // from a label, since the label must begin with two zero bits because
            // labels are restricted to 63 octets or less.  (The 10 and 01 combinations
            // are reserved for future use.)
            // Source: https://www.rfc-editor.org/rfc/rfc1035#section-4.1.4
            is_pointer = (buffer[pointer_cursor] & 0b11000000) == 0b11000000;
            if is_pointer {
                pointer_cursor = ((((buffer[pointer_cursor] & 0b00111111) as u16) << 8)
                    | (buffer[pointer_cursor + 1] as u16))
                    as usize;
                *cursor += 3;
            }
            let length = buffer[pointer_cursor] as usize;
            pointer_cursor += 1;
            let label_bytes = &buffer[pointer_cursor..pointer_cursor + length];
            pointer_cursor += length;
            let label_string = std::str::from_utf8(label_bytes).unwrap();
            names.push(String::from(label_string.to_string()))
        }
        if !is_pointer {
            *cursor = pointer_cursor + 1;
        }
        *cursor += 4;

        let question = DnsQuestion {
            names,
            qtype: 1,  
            qclass: 1, 
        };
        question
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();        
        for name in &self.names {
            bytes.push(name.len() as u8);
            bytes.extend_from_slice(name.as_bytes());
        }
        bytes.push(0); // Null byte to mark end of names        
        bytes.extend_from_slice(&self.qtype.to_big_endian());
        bytes.extend_from_slice(&self.qclass.to_big_endian());        
        bytes
    }
}