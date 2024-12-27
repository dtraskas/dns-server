use super::utils;
use utils::ToBigEndian;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum QueryResponse {
    Question = 0,
    Response = 1,
}

#[derive(Debug, Clone)]
pub struct DnsHeader {
    id: u16,
    pub qr: QueryResponse,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DnsHeader {
    pub fn new(buffer: Vec<u8>) -> Self {
        
        let opcode = (buffer[2] >> 3) & 0x0F;
        let header = DnsHeader {
            id: (buffer[0] as u16) << 8 | buffer[1] as u16,
            qr: match buffer[2] >> 7 {
                0 => QueryResponse::Question,
                1 => QueryResponse::Response,
                _ => unreachable!("Invalid value for a 1-bit field"),
            },
            opcode,
            aa: buffer[2] >> 2 & 0b1,
            tc: buffer[2] >> 1 & 0b1,
            rd: buffer[2] & 0b1,
            ra: buffer[3] >> 7 & 0b1,
            z: buffer[3] >> 4 & 0b111,
            rcode: buffer[3] & 0b1111,
            qdcount: (buffer[4] as u16) << 8 | buffer[5] as u16,
            ancount: (buffer[6] as u16) << 8 | buffer[7] as u16,
            nscount: (buffer[8] as u16) << 8 | buffer[9] as u16,
            arcount: (buffer[10] as u16) << 8 | buffer[11] as u16,
        };
        header
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];
        // Packet Identifier (ID)
        buffer.extend_from_slice(&self.id.to_big_endian());
        let qr = match self.qr {
            QueryResponse::Question => 0 as u8,
            QueryResponse::Response => 1 as u8,            
        } << 7;
        let opcode = (self.opcode & 0b1111) << 3;
        let aa = (self.aa as u8) << 2;
        let tc = (self.tc as u8) << 1;
        let rd = self.rd as u8;
        // Flags (QR, OPCODE, AA, TC, RD)
        buffer.push(qr | opcode | aa | tc | rd);

        // Flags (RA, Z, RCODE)
        buffer.push((self.ra << 7) | ((self.z & 0x7) << 4) | (self.rcode & 0xF));
        // Count Fields: QDCOUNT, ANCOUNT, NSCOUNT, ARCOUNT
        let counts = [
            self.qdcount,
            self.ancount,
            self.nscount,
            self.arcount,
        ];
    
        for count in counts.iter() {
            buffer.extend_from_slice(&count.to_big_endian());
        }
        buffer
    }    
}