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
}
#[derive(Default)]
struct Labels {
    counter: Vec<u8>,
    label: Vec<String>,
}

impl Labels {
    fn add_name(&mut self, name: String) {
        self.counter.push(name.len() as u8);
        self.label.push(name);
    }

    fn out(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (c, l) in self.counter.iter().zip(self.label.iter()) {
            out.push(*c as u8);
            out.extend_from_slice(l.as_bytes());
        }
        // termination
        out.push(0);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_label_encode_with_counter() {
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
}
