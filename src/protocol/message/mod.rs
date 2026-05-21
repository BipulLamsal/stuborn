use crate::protocol::{
    message::{
        header::{MessageHeader, MessageHeaderType},
        question::Question,
    },
    rr::{Labels, RRFormat},
};

/*
   +---------------------+
   |        Header       |
   +---------------------+
   |       Question      | the question for the name server
   +---------------------+
   |        Answer       | RRs answering the question
   +---------------------+
   |      Authority      | RRs pointing toward an authority
   +---------------------+
   |      Additional     | RRs holding additional information
   +---------------------+

*/

pub mod header;
pub mod question;

#[derive(Debug)]
pub struct DNSpacket {
    header: MessageHeader,
    question: Question,
    answer: RRFormat,
    authority: RRFormat,
    additional: RRFormat,
}

impl Default for DNSpacket {
    fn default() -> Self {
        Self {
            header: MessageHeader::default(),
            question: Question::default(),
            answer: RRFormat::new(0),
            authority: RRFormat::new(0),
            additional: RRFormat::new(0),
        }
    }
}

impl DNSpacket {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let mut header_buffer = [0u8; 12];

        let pointer_compression_callback = |start: usize| {
            let labels = Labels::from_buffer(&buffer[start..], |_: usize| {
                return None;
            });
            return Some(labels.0);
        };

        header_buffer.copy_from_slice(&buffer[0..12]);
        let mut header = MessageHeader::default();
        header.from(header_buffer);

        let mut qd = MessageHeaderType::QdCount(0);
        let mut an = MessageHeaderType::AnCount(0);
        let mut ns = MessageHeaderType::NsCount(0);
        let mut ar = MessageHeaderType::ArCount(0);

        header.get_message_header_value(&mut qd);
        header.get_message_header_value(&mut an);
        header.get_message_header_value(&mut ns);
        header.get_message_header_value(&mut ar);

        let mut question = Question::default();
        let mut start = 12;

        start += question.from_buffer(&buffer[12..]);

        let mut answer = RRFormat::new(an.get_count());
        start += answer.from_buffer(&buffer[start..], pointer_compression_callback);

        let mut authority = RRFormat::new(ns.get_count());
        start += authority.from_buffer(&buffer[start..], pointer_compression_callback);

        let mut additional = RRFormat::new(ar.get_count());
        let _ = additional.from_buffer(&buffer[start..], pointer_compression_callback);

        Self {
            header,
            question,
            answer,
            authority,
            additional,
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(self.header.to_buffer());
        out.extend_from_slice(&self.question.to_buffer());
        out.extend_from_slice(&self.answer.to_buffer());
        out.extend_from_slice(&self.authority.to_buffer());
        out.extend_from_slice(&self.additional.to_buffer());
        out
    }
}
