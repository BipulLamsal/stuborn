use std::fmt;
use std::io::Cursor;
/// into represnts the postion (offset bit index) of the buffer
/// i am thinking ths way its easier to get the correct position and also parsing this would be
/// easier
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MessageHeaderType {
    /// A 16 bit identifier assigned by the program that
    /// generates any kind of query.  This identifier is copied
    /// the corresponding reply and can be used by the requester
    /// to match up replies to outstanding queries.
    Id(u16) = 0,
    /// one bit field that specifies whether this is query or message
    Qr(bool) = 16,
    /// 4 bit field that specifies kind of query in this message
    OpCode(Opcode) = 17,
    /// Authorative Answer : responding name server is an authority for the domain name
    /// in question section (1 bit)
    AA(bool) = 21,

    /// TrunCation - specifies that this message was truncated
    TC(bool) = 22,
    /// attempt to resolve the query recursively if it does not have an answer readily available.
    RD(bool) = 23,
    /// whether or not recursive queries are allowed
    RA(bool) = 24,
    /// must be zero in all queries and response (3 bits for now) reserved
    Z(u8) = 25,
    /// Response Code : 4 bit field
    Rcode(ResponseCode) = 28,

    /// Number of entries in question section
    QdCount(u16) = 32,

    /// Number of entries in Answer section
    AnCount(u16) = 48,

    /// Number of name server resource records in the authority records
    NsCount(u16) = 64,

    /// Number of name server resource records in the additional section
    ArCount(u16) = 80,
}

impl MessageHeaderType {
    pub fn get_count(&self) -> u16 {
        match self {
            &MessageHeaderType::QdCount(v)
            | &MessageHeaderType::AnCount(v)
            | &MessageHeaderType::NsCount(v)
            | &MessageHeaderType::ArCount(v) => v,
            _ => 0,
        }
    }
}

impl From<MessageHeaderType> for usize {
    fn from(value: MessageHeaderType) -> Self {
        // Saftey : repr of MessageHeaderType is u8 so we are typecasting to u8 first and then usize
        // no direct conversion
        unsafe {
            let ptr_header = &value as *const MessageHeaderType;
            let ptr_usize = ptr_header as *const u8;
            *ptr_usize as usize
        }
    }
}

#[derive(Default)]
pub struct MessageHeader {
    cursor: Cursor<[u8; 12]>,
}

impl fmt::Debug for MessageHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut id = MessageHeaderType::Id(0);
        let mut qr = MessageHeaderType::Qr(false);
        let mut opcode = MessageHeaderType::OpCode(Opcode::Query);
        let mut aa = MessageHeaderType::AA(false);
        let mut tc = MessageHeaderType::TC(false);
        let mut rd = MessageHeaderType::RD(false);
        let mut ra = MessageHeaderType::RA(false);
        let mut rcode = MessageHeaderType::Rcode(ResponseCode::NoErr);
        let mut qd = MessageHeaderType::QdCount(0);
        let mut an = MessageHeaderType::AnCount(0);
        let mut ns = MessageHeaderType::NsCount(0);
        let mut ar = MessageHeaderType::ArCount(0);

        self.get_message_header_value(&mut id);
        self.get_message_header_value(&mut qr);
        self.get_message_header_value(&mut opcode);
        self.get_message_header_value(&mut aa);
        self.get_message_header_value(&mut tc);
        self.get_message_header_value(&mut rd);
        self.get_message_header_value(&mut ra);
        self.get_message_header_value(&mut rcode);
        self.get_message_header_value(&mut qd);
        self.get_message_header_value(&mut an);
        self.get_message_header_value(&mut ns);
        self.get_message_header_value(&mut ar);

        f.debug_struct("MessageHeader")
            .field("id", &format!("0x{:04X}", id.get_count()))
            .field("qr", &matches!(qr, MessageHeaderType::Qr(true)))
            .field("opcode", &opcode)
            .field("aa", &matches!(aa, MessageHeaderType::AA(true)))
            .field("tc", &matches!(tc, MessageHeaderType::TC(true)))
            .field("rd", &matches!(rd, MessageHeaderType::RD(true)))
            .field("ra", &matches!(ra, MessageHeaderType::RA(true)))
            .field("rcode", &rcode)
            .field("qdcount", &qd.get_count())
            .field("ancount", &an.get_count())
            .field("nscount", &ns.get_count())
            .field("arcount", &ar.get_count())
            .finish()
    }
}

impl MessageHeader {
    /// Internal helper to set or clear a single bit at a specific position
    fn set_flag(&mut self, pos: usize, rem: usize, value: bool) {
        let buf = self.cursor.get_mut();
        let mask = 1 << (7 - rem);
        if value {
            buf[pos] |= mask;
        } else {
            buf[pos] &= !mask;
        }
    }

    /// This is essential if some known buffer is passed in (eg: response)
    /// and extracted via get_message_header_value()
    pub fn from(&mut self, buffer: [u8; 12]) {
        *(self.cursor.get_mut()) = buffer;
    }

    pub fn to_buffer(&self) -> &[u8] {
        self.cursor.get_ref()
    }

    pub fn add(&mut self, header_type: MessageHeaderType) -> &mut Self {
        let bit_idx: usize = header_type.into();
        let pos = bit_idx / 8;
        let rem = bit_idx % 8;
        let cursor = self.cursor.get_mut();

        match header_type {
            MessageHeaderType::Id(v)
            | MessageHeaderType::QdCount(v)
            | MessageHeaderType::AnCount(v)
            | MessageHeaderType::NsCount(v)
            | MessageHeaderType::ArCount(v) => {
                // since they are u16 and divided into two different parts
                let be_bytes = v.to_be_bytes();
                cursor[pos] = be_bytes[0];
                cursor[pos + 1] = be_bytes[1];
            }

            MessageHeaderType::Qr(v) => self.set_flag(pos, rem, v),
            MessageHeaderType::AA(v) => self.set_flag(pos, rem, v),
            MessageHeaderType::TC(v) => self.set_flag(pos, rem, v),
            MessageHeaderType::RD(v) => self.set_flag(pos, rem, v),
            MessageHeaderType::RA(v) => self.set_flag(pos, rem, v),

            MessageHeaderType::OpCode(v) => {
                // technically its 3 : 4-rem
                cursor[pos] |= (v as u8) << (4 - rem);
            }
            MessageHeaderType::Z(v) => {
                // the approach here is quite simple
                // z is typically 0 we could hardcode it but
                // we only need the 3 bit and we shift by 4 leaving the first bit as it is
                cursor[pos] |= (v & 0x07) << 4;
            }
            MessageHeaderType::Rcode(v) => {
                cursor[pos] |= (v as u8) & 0x0F;
            }
        }
        self
    }
    /// Internal function to get the u16 value at byte offset `start`
    fn get_u16_value(&self, start: usize) -> u16 {
        let slice = self.cursor.get_ref();
        u16::from_be_bytes([slice[start], slice[start + 1]])
    }

    /// Internal function to get a single bit value at byte offset `start`, bit remainder `rem`
    fn get_bool_value(&self, start: usize, rem: usize) -> bool {
        let slice = self.cursor.get_ref();
        // shift right so the target bit is at position 0, then mask it
        (slice[start] >> (7 - rem)) & 1 == 1
    }

    pub fn get_message_header_value(&self, message_type: &mut MessageHeaderType) {
        let bit_idx: usize = (*message_type).into();
        let pos = bit_idx / 8;
        let rem = bit_idx % 8;

        match message_type {
            MessageHeaderType::Id(v)
            | MessageHeaderType::QdCount(v)
            | MessageHeaderType::AnCount(v)
            | MessageHeaderType::NsCount(v)
            | MessageHeaderType::ArCount(v) => {
                *v = self.get_u16_value(pos);
            }

            MessageHeaderType::Qr(v)
            | MessageHeaderType::AA(v)
            | MessageHeaderType::TC(v)
            | MessageHeaderType::RD(v)
            | MessageHeaderType::RA(v) => {
                *v = self.get_bool_value(pos, rem);
            }

            MessageHeaderType::OpCode(v) => {
                let slice = self.cursor.get_ref();
                let raw = slice[pos] >> 3 & 0x0F;
                // SAFETY: OpCode variants map directly to their u8 discriminants
                *v = unsafe { std::mem::transmute(raw) };
            }

            MessageHeaderType::Z(v) => {
                let slice = self.cursor.get_ref();
                *v = (slice[pos] >> 4) & 0x07;
            }

            MessageHeaderType::Rcode(v) => {
                let slice = self.cursor.get_ref();
                let raw = slice[pos] & 0x0F;
                // SAFETY: ResponseCode variants map directly to their u8 discriminants
                *v = unsafe { std::mem::transmute(raw) };
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Opcode {
    Query = 0,
    InverseQuery = 1,
    Status = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum ResponseCode {
    /// No Error Condition
    NoErr = 0,
    /// Name Server Unable to Interpret Query
    FormatErr = 1,
    /// Name Server Unable to process due to server problem
    ServerFailure = 2,
    /// domain name doesnot exist
    NameError = 3,
    /// server couldnot process such kind of query  
    NotImplemented = 4,
    /// refuses to perform the specified operation for policy reasons
    Refused = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// this is random test because i always forget the be and le
    #[test]
    fn test_protocol_u16_be_bytes_test() {
        let value: u16 = 0x1234;
        //00000000 00001010
        let be_bytes = value.to_be_bytes();

        assert_eq!(be_bytes[0], 0x12);
        assert_eq!(be_bytes[1], 0x34);
    }

    #[test]
    fn test_protocol_header_generation() {
        let mut header = MessageHeader::default();
        header
            .add(MessageHeaderType::Id(0x1234))
            .add(MessageHeaderType::Qr(true))
            .add(MessageHeaderType::OpCode(Opcode::Query))
            .add(MessageHeaderType::QdCount(1));

        let result = header.cursor.into_inner();

        assert_eq!(result[0], 0x12);
        assert_eq!(result[1], 0x34);
        assert_eq!(result[2], 0x80);
        assert_eq!(result[4], 0x00);
        assert_eq!(result[5], 0x01);
    }

    #[test]
    fn test_protocol_header_id_retrieval() {
        let mut header = MessageHeader::default();
        header
            .add(MessageHeaderType::Id(0x1234))
            .add(MessageHeaderType::Qr(true))
            .add(MessageHeaderType::OpCode(Opcode::Query))
            .add(MessageHeaderType::QdCount(1));

        let mut id = MessageHeaderType::Id(0);
        header.get_message_header_value(&mut id);

        match id {
            MessageHeaderType::Id(v) => {
                assert_eq!(v, 0x1234);
            }
            _ => {}
        }
    }

    #[test]
    fn test_protocol_header_qr_retrieval() {
        let mut header = MessageHeader::default();
        header.add(MessageHeaderType::Qr(true));
        let mut qr = MessageHeaderType::Qr(false);
        header.get_message_header_value(&mut qr);
        match qr {
            MessageHeaderType::Qr(v) => assert_eq!(v, true),
            _ => {}
        }
    }

    #[test]
    fn test_protocol_header_aa_retrieval() {
        let mut header = MessageHeader::default();
        header.add(MessageHeaderType::AA(true));
        let mut aa = MessageHeaderType::AA(false);
        header.get_message_header_value(&mut aa);
        match aa {
            MessageHeaderType::AA(v) => assert_eq!(v, true),
            _ => {}
        }
    }

    #[test]
    fn test_protocol_header_tc_retrieval() {
        let mut header = MessageHeader::default();
        header.add(MessageHeaderType::TC(true));
        let mut tc = MessageHeaderType::TC(false);
        header.get_message_header_value(&mut tc);
        match tc {
            MessageHeaderType::TC(v) => assert_eq!(v, true),
            _ => {}
        }
    }

    #[test]
    fn test_protocol_header_opcode_retrieval() {
        let mut header = MessageHeader::default();
        header.add(MessageHeaderType::OpCode(Opcode::Status));
        let mut opcode = MessageHeaderType::OpCode(Opcode::Query);
        header.get_message_header_value(&mut opcode);
        match opcode {
            MessageHeaderType::OpCode(v) => assert_eq!(v as u8, Opcode::Status as u8),
            _ => {}
        }
    }
}
