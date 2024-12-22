use std::net::UdpSocket;

trait ToBigEndian {
    fn to_big_endian(&self) -> Vec<u8>;
}

// Implement the trait for `u16`.
impl ToBigEndian for u16 {
    fn to_big_endian(&self) -> Vec<u8> {
        vec![
            (*self >> 8) as u8,    // Most significant byte
            (*self & 0xFF) as u8, // Least significant byte
        ]
    }
}

// Implement the trait for `u32`.
impl ToBigEndian for u32 {
    fn to_big_endian(&self) -> Vec<u8> {
        vec![
            (*self >> 24) as u8,    // Most significant byte
            (*self >> 16) as u8,   // Second byte
            (*self >> 8) as u8,    // Third byte
            (*self & 0xFF) as u8, // Least significant byte
        ]
    }
}

struct DnsHeader {
    id: u16,
    qr: u8,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DnsHeader {
    fn new(id: u16) -> Self {
        Self {
            id,
            qr: 1,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode:0,
            qdcount: 1,
            ancount: 1,
            nscount: 0,
            arcount: 0,
        }
    }
    
    fn to_bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];
    
        // Packet Identifier (ID)
        bytes[..2].copy_from_slice(&self.id.to_big_endian());
    
        // Flags (QR, OPCODE, AA, TC, RD)
        bytes[2] = (self.qr << 7)
            | ((self.opcode & 0xF) << 3)
            | ((self.aa & 0x1) << 2)
            | ((self.tc & 0x1) << 1)
            | (self.rd & 0x1);
    
        // Flags (RA, Z, RCODE)
        bytes[3] = (self.ra << 7) | ((self.z & 0x7) << 4) | (self.rcode & 0xF);
    
        // Count Fields: QDCOUNT, ANCOUNT, NSCOUNT, ARCOUNT
        let counts = [
            self.qdcount,
            self.ancount,
            self.nscount,
            self.arcount,
        ];
    
        for (i, count) in counts.iter().enumerate() {
            bytes[4 + i * 2..6 + i * 2].copy_from_slice(&count.to_big_endian());
        }
        bytes
    }    
}

struct DnsQuestion {
    name: String,
    qtype: u16,
    qclass: u16,
}

impl DnsQuestion {
    fn new(domain_name: &str) -> Self {
        Self {
            name: domain_name.to_string(),
            qtype: 1,
            qclass: 1,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();        
        let labels: Vec<&str> = self.name.split('.').collect();
        for label in labels {
            buffer.push(label.len() as u8);
            buffer.extend_from_slice(label.as_bytes());
        }
        buffer.push(0);
        buffer.extend_from_slice(&self.qtype.to_big_endian());
        buffer.extend_from_slice(&self.qclass.to_big_endian());
        buffer
    }
}

struct DnsAnswer {
    name: String,
    atype: u16,
    aclass: u16,
    ttl: u32,
    rdlength: u16,
    rdata: String,
}

impl DnsAnswer {
    fn new(domain_name: &str, ip_address: &str) -> Self {
        Self {
            name: domain_name.to_string(),
            atype: 1,
            aclass: 1,
            ttl: 60,
            rdlength: 4,
            rdata: ip_address.to_string(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let labels: Vec<&str> = self.name.split('.').collect();
        for label in labels {
            buffer.push(label.len() as u8);
            buffer.extend_from_slice(label.as_bytes());
        }
        buffer.push(0);
        buffer.extend_from_slice(&self.atype.to_big_endian());
        buffer.extend_from_slice(&self.aclass.to_big_endian());
        buffer.extend_from_slice(&self.ttl.to_big_endian());
        buffer.extend_from_slice(&self.rdlength.to_big_endian());

        let parts: Vec<u8> = self.rdata.split('.').map(|s| s.parse().unwrap()).collect();

        // Combine the parts into a 32-bit u32 value
        let ip_u32 = (parts[0] as u32) << 24
                | (parts[1] as u32) << 16
                | (parts[2] as u32) << 8
                | (parts[3] as u32);

        buffer.extend_from_slice(&ip_u32.to_big_endian());
        buffer
    }
}

struct DnsMessage {
    header: DnsHeader,
    question: DnsQuestion,    
    answer: DnsAnswer,
}

impl DnsMessage {
    fn new(id: u16, domain_name: &str, ip_address: &str) -> Self {
        Self {
            header: DnsHeader::new(id),
            question: DnsQuestion::new(domain_name),
            answer: DnsAnswer::new(domain_name, ip_address),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.question.to_bytes());
        bytes.extend_from_slice(&self.answer.to_bytes());
        bytes
    }
}


fn main() {
    println!("Logs from your program will appear here!");
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let id = u16::from_be_bytes([buf[0], buf[1]]);
                
                let mut message = DnsMessage::new(id, "codecrafters.io", "8.8.8.8");
                message.header.opcode = (buf[2] >> 3) & 0x0F;                
                message.header.rd = (buf[2] >> 0) & 0x0F;                                
                message.header.rcode = if message.header.opcode == 0 { 0 } else { 4 };

                udp_socket.send_to(&message.to_bytes(), source).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}