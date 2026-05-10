// Resolve Record
#[derive(Debug, PartialEq)]
pub struct RRFormat {
    /// Owner Name; name of the node to which record persists
    name: Labels,
    /// 2 bytes of data containing one of the RR type codes
    rr_type: RRType,
    /// 2 bytes of data containing one of the RR class codes
    rr_class: RRClass,
    /// 4 bytes of time data; zero value for volatile data (SOA record)  
    ttl: u32,
    /// length
    rdlen: u16,
    ///  variable length string that describes the resource
    rdata: String,

    // just to impl the from_buffer trait
    num_answers: u8,
}

impl Default for RRFormat {
    fn default() -> Self {
        Self {
            name: Labels::default(),
            rr_type: RRType::A,
            rr_class: RRClass::IN,
            ttl: 0,
            rdlen: 0,
            rdata: String::new(),
            num_answers: 0,
        }
    }
}

impl RRFormat {
    pub fn new(num: u8) -> Self {
        let mut value = Self::default();
        value.num_answers = num;
        return value;
    }

    fn to_buffer(&self) -> Vec<u8> {
        let name_slice = self.name.out();
        let rr_type_slice = (self.rr_type as u16).to_be_bytes();
        let rr_class_slice = (self.rr_class as u16).to_be_bytes();
        let ttl_slice = self.ttl.to_be_bytes();
        let rdlen_slice = self.rdlen.to_be_bytes();
        let rdata_slice = self.rdata.as_bytes();
        let mut vec = Vec::new();

        vec.extend_from_slice(&name_slice);
        vec.extend_from_slice(&rr_type_slice);
        vec.extend_from_slice(&rr_class_slice);
        vec.extend_from_slice(&ttl_slice);
        vec.extend_from_slice(&rdlen_slice);
        vec.extend_from_slice(rdata_slice);

        return vec;
    }
    fn from_buffer(&mut self, buffer: &[u8]) {
        let (labels, mut start) = Labels::from_buffer(buffer, self.num_answers);
        start += 1;

        let rr_type = u16::from_be_bytes([buffer[start], buffer[start + 1]]);
        let rr_type = RRType::try_from(rr_type).unwrap();
        start += 2;

        let rr_class = u16::from_be_bytes([buffer[start], buffer[start + 1]]);
        let rr_class = RRClass::try_from(rr_class).unwrap();
        start += 2;

        let ttl = u32::from_be_bytes([
            buffer[start],
            buffer[start + 1],
            buffer[start + 2],
            buffer[start + 3],
        ]);
        start += 4;

        let rdlen = u16::from_be_bytes([buffer[start], buffer[start + 1]]);
        start += 2;

        let rdata = String::from_utf8(buffer[start..start + rdlen as usize].to_vec()).unwrap();

        self.name = labels;
        self.rr_type = rr_type;
        self.rr_class = rr_class;
        self.ttl = ttl;
        self.rdlen = rdlen;
        self.rdata = rdata;
    }
}

// Qtype appears in the question part of a query
// Qtype are a superset of RRType, hence all RRTypes are valid Qtypes.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Qtype {
    Other(RRType),
    AXFR = 252,
    ALL = 255,
    // there are MAILB & MAILA which i am not implementing here
}

/// TYPE fields are used in resource records, they are sub field of the Qtype
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RRType {
    /// a host address
    A = 1,
    /// an authoritative name server
    NS = 2,
    /// the canonical name for an alias
    CNAME = 5,
    /// marks the start of a zone of authority
    SOA = 6,
    /// a domain name pointer
    PTR = 12,
    /// text strings
    TXT = 16,
}

impl TryFrom<u16> for RRType {
    type Error = u16;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::A),
            2 => Ok(Self::NS),
            5 => Ok(Self::CNAME),
            6 => Ok(Self::SOA),
            12 => Ok(Self::PTR),
            16 => Ok(Self::TXT),
            _ => Err(v),
        }
    }
}

/// CLASS fields appear in resource records.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RRClass {
    /// The internet (mostly used)
    IN = 1,
    /// The CHAOS class
    CH = 3,
    /// Hesiod
    HS = 4,
    /// QCLASS fields appear in the question section of a query.
    ALL = 255,
}

impl TryFrom<u16> for RRClass {
    type Error = u16;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::IN),
            3 => Ok(Self::CH),
            4 => Ok(Self::HS),
            255 => Ok(Self::ALL),
            _ => Err(v),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Labels {
    label: Vec<String>,
}

impl Labels {
    pub fn add_name(&mut self, name: String) {
        self.label.push(name);
    }

    pub fn out(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for l in &self.label {
            out.push(l.len() as u8);
            out.extend_from_slice(l.as_bytes());
        }
        // termination
        out.push(0);
        out
    }

    pub fn get_len(&self) -> usize {
        self.label.len()
    }

    pub fn from_buffer(buffer: &[u8], iter_len: u8) -> (Self, usize) {
        let mut labels = Labels::default();
        let mut start = 0_usize;

        for _ in 0..iter_len {
            let num = buffer[start] as usize;
            let slice = &buffer[start + 1..start + 1 + num];
            let string_value = str::from_utf8(slice).expect("Expected Str in the byte");
            labels.add_name(string_value.to_string());
            start += num + 1;
        }

        (labels, start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rr() -> RRFormat {
        let mut name = Labels::default();
        name.add_name("www".to_string());
        name.add_name("google".to_string());
        name.add_name("com".to_string());

        RRFormat {
            name,
            rr_type: RRType::A,
            rr_class: RRClass::IN,
            ttl: 300,
            rdlen: 9,
            rdata: "127.0.0.1".to_string(),
            num_answers: 3,
        }
    }

    #[test]
    fn test_resolve_record_encode() {
        let rr = make_rr();
        let buffer = rr.to_buffer();

        assert_eq!(buffer[0], 3);
        assert_eq!(&buffer[1..4], b"www");
        assert_eq!(buffer[4], 6);
        assert_eq!(&buffer[5..11], b"google");
        assert_eq!(buffer[11], 3);
        assert_eq!(&buffer[12..15], b"com");
        assert_eq!(buffer[15], 0); // null terminator

        // rr_type: A = 1
        assert_eq!(&buffer[16..18], &[0, 1]);
        // rr_class: IN = 1
        assert_eq!(&buffer[18..20], &[0, 1]);
        // ttl: 300
        assert_eq!(&buffer[20..24], &[0, 0, 1, 44]);
        // rdlen: 9
        assert_eq!(&buffer[24..26], &[0, 9]);
        // rdata
        assert_eq!(&buffer[26..], b"127.0.0.1");
    }

    #[test]
    fn test_resolve_record_decode() {
        let original = make_rr();
        let buffer = original.to_buffer();

        // expectation of num of answer is 3
        let mut decoded = RRFormat::new(3);
        decoded.from_buffer(&buffer);

        assert_eq!(decoded, original);
    }
}
