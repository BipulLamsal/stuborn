use std::io::Cursor;
/// into represnts the postion (offset bit index) of the buffer
/// i am thinking ths way its easier to get the correct position and also parsing this would be
/// easier
#[repr(u8)]
#[derive(Debug)]
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
    AA(u8) = 21,

    /// TrunCation - specifies that this message was truncated
    TC(u8) = 22,
    /// attempt to resolve the query recursively if it does not have an answer readily available.
    RD(u8) = 23,
    /// whether or not recursive queries are allowed
    RA(u8) = 24,
    /// must be zero in all queries and response (3 bits for now) reserved
    Z(u8) = 25,
    /// Response Code : 4 bit field
    Rcode(ResponseCode) = 26,

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
    fn add(&mut self, header_type: MessageHeaderType) -> &mut Self {
        let pos: usize = header_type.into() / 8;
        let rem: usize = header_type.into() % 8;
        let cursor = self.cursor.get_mut();
        match header_type {
            MessageHeaderType::Id(value)
            | MessageHeaderType::QdCount(value)
            | MessageHeaderType::AnCount(value)
            | MessageHeaderType::NsCount(value)
            | MessageHeaderType::ArCount(value) => {
                /*
                let first = value >> 8;
                let second = value & 0xFF;
                // first byte goes first in be bytes
                idk[pos] = first as u8;
                idk[pos + 1] = second as u8;
                */

                let be_bytes: [u8; 2] = value.to_be_bytes();
                cursor[pos] = be_bytes[0];
                cursor[pos + 1] = be_bytes[1];
                self
            }
            MessageHeaderType::Qr(value) => {
                if value {
                    cursor[pos] = (1 >> 7) as u8;
                } else {
                    cursor[pos] = (0 >> 7) as u8;
                }
                self
            }
            MessageHeaderType::OpCode(value) => {
                let opcode = value as u8;
                cursor[pos] = cursor[pos] | (opcode >> 4);
                self
            }
            MessageHeaderType::AA(value) => {}
            MessageHeaderType::TC(value) => {}
            MessageHeaderType::RD(value) => {}
            MessageHeaderType::RA(value) => {}
            MessageHeaderType::Z(value) => {}
            MessageHeaderType::Rcode(value) => {}
            MessageHeaderType::QdCount(value) => {}
            MessageHeaderType::AnCount(value) => {}
            MessageHeaderType::NsCount(value) => {}
            MessageHeaderType::ArCount(value) => {}
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
enum Opcode {
    Query = 0,
    InverseQuery = 1,
    Status = 2,
}

#[repr(u8)]
#[derive(Debug)]
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
    fn header() {
        let packet = MessageHeader::builder().add(header_type);
    }
}
