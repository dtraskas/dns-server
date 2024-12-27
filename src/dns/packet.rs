use super::header::DnsHeader;
use super::question::DnsQuestion;
use super::answer::DnsAnswer;

#[derive(Debug, Clone)]
pub struct DnsPacket{
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsAnswer>, 
}

impl DnsPacket {
    pub fn new(buffer: &[u8]) -> Self {
        let header = DnsHeader::new(buffer.to_vec());
        let answer_count = header.ancount as usize;
        let question_count = header.qdcount as usize;
        let mut cursor = 12;
        Self {
            header,
            questions: (0..question_count)
            .map(|_| {
                DnsQuestion::new(buffer.to_vec(), &mut cursor)
            })
            .collect(),
            answers: (0..answer_count)
            .map(|_| {
                DnsAnswer::new(buffer.to_vec(), &mut cursor)
            })
            .collect(),            
        }
    }

    pub fn split(&self) -> Vec<DnsPacket> {
        let mut header = self.header.clone();
        header.qdcount = 1;
        return self.questions.clone().into_iter()
            .map(|question| DnsPacket {
                header: header.clone(),
                questions: vec![question],
                answers: vec![],
            })
            .collect();
    }

    pub fn merge(dns_packets: Vec<DnsPacket>) -> DnsPacket {
        let mut header = dns_packets[0].header.clone();
        header.qdcount = dns_packets.len() as u16;
        header.ancount = dns_packets.len() as u16;

        let mut questions: Vec<DnsQuestion> = vec![];
        let mut answers: Vec<DnsAnswer> = vec![];
        dns_packets.into_iter().for_each(|packet| {
            questions.extend(packet.questions);
            answers.extend(packet.answers);
        });

        DnsPacket {header, questions, answers}
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.header.to_bytes();
        for dns_question in self.questions.iter() {
            bytes.extend(dns_question.to_bytes());
        }
        for dns_answer in self.answers.iter() {
            bytes.extend(dns_answer.to_bytes());
        }
        bytes
    }
}