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

    // just to impl the from_buffer trait
    num_questions: u8,
}

impl Default for Question {
    fn default() -> Self {
        Self {
            qname: Labels::default(),
            q_type: RRType::A,
            class: RRClass::IN,
            num_questions: 0,
        }
    }
}

impl Question {
    pub fn new(num: u8) -> Self {
        let mut value = Question::default();
        value.num_questions = num;
        return value;
    }

    pub fn get_num(&self) -> u8 {
        self.num_questions
    }

    pub fn add_name(&mut self, name: String) {
        self.qname.add_name(name);
        if self.qname.counter.len() > self.num_questions as usize {
            self.num_questions += 1;
        }
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

    /// Should be called after constructing with `new(num)` where `num` is the number of questions.
    pub fn from_buffer(&mut self, buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut labels = Labels::default();
        let mut start = 0_usize;

        for _ in 0..self.num_questions {
            let num = buffer[start] as usize;
            let slice = &buffer[start + 1..start + 1 + num];
            let string_value = str::from_utf8(slice)?;
            labels.add_name(string_value.to_string());
            start += num + 1;
        }
        self.qname = labels;

        start += 1; // null terminator

        let rr_type = u16::from_be_bytes([buffer[start], buffer[start + 1]]);
        let class = u16::from_be_bytes([buffer[start + 2], buffer[start + 3]]);

        // instead of transmute lets use try from fresh fresh implemenation
        self.q_type = RRType::try_from(rr_type).expect("Unknown RR type: {rr_type}");
        self.class = RRClass::try_from(class).expect("Unknown class: {class}");

        Ok(())
    }
}
#[derive(Default, Debug, PartialEq)]
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
        let mut decoded = Question::new(question.get_num());
        decoded.from_buffer(&result).unwrap();

        assert_eq!(question, decoded);
    }
}
