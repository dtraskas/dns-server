#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DnsAnswer {
    pub name: Vec<String>,
    pub atype: u16,
    pub aclass: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl DnsAnswer {
    pub fn new(buffer: Vec<u8>, cursor: &mut usize) -> Self {
        let mut labels: Vec<String> = vec![];
        let mut pointer_cursor: usize = *cursor;
        let mut is_pointer: bool = false;

        while buffer[pointer_cursor] != 0x00 {
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
            labels.push(String::from(label_string.to_string()))
        }
        if !is_pointer {
            *cursor = pointer_cursor + 1;
        }
        *cursor += 8;        
        Self {
            name: labels,
            atype: 1,  // Type A (IPv4 address)
            aclass: 1, // Class IN
            ttl: 60,   
            rdlength: 4, // IPv4 address length
            rdata: vec![buffer[*cursor + 2],buffer[*cursor + 3],buffer[*cursor + 4],buffer[*cursor + 5]],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![]; 
        for label in &self.name {
            bytes.push(label.len() as u8);
            bytes.extend_from_slice(label.as_bytes());
        }
        let mut bytes: Vec<u8> =
            self.name.iter().flat_map(|label| self.encode(label)).collect();
        bytes.extend(vec![0x00]);
        bytes.extend(vec![0, 1, 0, 1]);
        bytes.extend(Vec::from(self.ttl.to_be_bytes()));
        bytes.extend(Vec::from(self.rdlength.to_be_bytes()));
        bytes.extend(self.rdata.clone());
        bytes
    }

    fn encode(&self, content: &String) -> Vec<u8> {
        let mut encoded_label = vec![content.len() as u8];
        encoded_label.extend(content.chars().flat_map(|c| c.to_string().into_bytes()).collect::<Vec<u8>>());
        encoded_label
    }
}