use crate::protocol::rr::Labels;
use crate::protocol::rr::RRClass;
use crate::protocol::rr::RRType;
/*                                 1  1  1  1  1  1
  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                                               |
/                     QNAME                     /
/                                               /
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                     QTYPE                     |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                     QCLASS                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

*/

/// carry the "question" in most queries,
/// contains QDCOUNT (usually 1) entries,
#[derive(Debug, PartialEq)]
pub struct Question {
    // this can be of variable size a domain name represented as a sequence of labels, where each label consists of a length octet followed by that number of octets.
    qname: Labels,
    q_type: RRType,
    class: RRClass,
}

impl Default for Question {
    fn default() -> Self {
        Self {
            qname: Labels::default(),
            q_type: RRType::A,
            class: RRClass::IN,
        }
    }
}

impl Question {
    pub fn new(_num: u16) -> Self {
        Self::default()
    }

    pub fn add_name(&mut self, name: String) {
        self.qname.add_name(name);
    }

    pub fn set_type(&mut self, q_type: RRType) {
        self.q_type = q_type;
    }

    pub fn set_class(&mut self, q_class: RRClass) {
        self.class = q_class;
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = self.qname.out();
        let q_type = self.q_type as u16; // 2 bytes
        let q_class = self.class as u16; // 2 bytes
        buffer.extend_from_slice(&q_type.to_be_bytes());
        buffer.extend_from_slice(&q_class.to_be_bytes());
        buffer
    }

    /// Returns the position after to the next item to start consuming the buffer
    pub fn from_buffer(&mut self, buffer: &[u8]) -> usize {
        let (label, start) = Labels::from_buffer(&buffer, |_: usize| None);
        self.qname = label;
        let rr_type = u16::from_be_bytes([buffer[start], buffer[start + 1]]);
        let class = u16::from_be_bytes([buffer[start + 2], buffer[start + 3]]);
        // instead of transmute lets use try from fresh fresh implemenation
        self.q_type = RRType::try_from(rr_type).expect("Unknown RR type: {rr_type}");
        self.class = RRClass::try_from(class).expect("Unknown class: {class}");
        return start + 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_encode_with_counter() {
        // setup
        let strings = ["www"];
        let mut question = Question::default();
        for item in strings {
            question.add_name(String::from(item));
        }
        // excercise
        let result = question.to_buffer();
        // validation
        assert_eq!(result[0], 3);
        assert_eq!(result[6], 1);
        assert_eq!(result[8], 1);
    }

    #[test]
    fn test_question_decode_from_buffer() {
        // setup
        let strings = ["www", "google", "com"];
        let mut question = Question::default();
        for item in strings {
            question.add_name(String::from(item));
        }
        // excercise
        let result = question.to_buffer();
        // verification
        let mut decoded = Question::default();
        let _ = decoded.from_buffer(&result);
        assert_eq!(question, decoded);
    }
}
