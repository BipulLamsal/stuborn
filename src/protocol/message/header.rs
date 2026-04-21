use std::io::Cursor;
/// into represnts the postion (offset bit index) of the buffer
/// i am thinking ths way its easier to get the correct position and also parsing this would be
/// easier
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum MessageHeaderType {
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

#[derive(Debug)]
struct MessageHeader {
    cursor: Cursor<[u8; 12]>,
}

impl MessageHeader {
    fn builder() -> Self {
        Self {
            // in total we have 12 * 8 = 96 bits
            cursor: Cursor::new([0u8; 12]),
        }
    }

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

    #[test]
    fn test_protocol_header_generation() {
        let mut header = MessageHeader::builder();
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
}
