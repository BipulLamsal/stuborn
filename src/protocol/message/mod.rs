use bit_vec::BitVec;

use crate::protocol::{
    message::{header::MessageHeader, question::Question},
    rr::RR_Format,
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

pub mod answer;
pub mod header;
pub mod question;

trait Packet {
    fn from_buffer(&self, buffer: [u8]) -> Self;
    fn to_buffer(&self) -> [u8];
}

struct DNSpacket {
    header: MessageHeader,
    question: Question,
    answer: RR_Format,
    authority: RR_Format,
    additional: RR_Format,
}

impl DNSpacket {}
