// A single Resource Record
#[derive(Debug, PartialEq)]
pub struct RRRecord {
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
    rdata: Vec<u8>,
}

impl Default for RRRecord {
    fn default() -> Self {
        Self {
            name: Labels::default(),
            rr_type: RRType::A,
            rr_class: RRClass::IN,
            ttl: 0,
            rdlen: 0,
            rdata: Vec::new(),
        }
    }
}

impl RRRecord {
    pub fn from_buffer(buffer: &[u8]) -> (Self, usize) {
        let (labels, mut pos) = Labels::from_buffer(buffer);

        let rr_type = u16::from_be_bytes([buffer[pos], buffer[pos + 1]]);
        let rr_type = RRType::try_from(rr_type).unwrap();
        pos += 2;

        let rr_class = u16::from_be_bytes([buffer[pos], buffer[pos + 1]]);
        let rr_class = RRClass::try_from(rr_class).unwrap();
        pos += 2;

        let ttl = u32::from_be_bytes([
            buffer[pos],
            buffer[pos + 1],
            buffer[pos + 2],
            buffer[pos + 3],
        ]);
        pos += 4;

        let rdlen = u16::from_be_bytes([buffer[pos], buffer[pos + 1]]);
        pos += 2;

        let rdata = Vec::from(&buffer[pos..pos + rdlen as usize]);
        pos += rdlen as usize;

        (
            Self {
                name: labels,
                rr_type,
                rr_class,
                ttl,
                rdlen,
                rdata,
            },
            pos,
        )
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend_from_slice(&self.name.out());
        vec.extend_from_slice(&(self.rr_type as u16).to_be_bytes());
        vec.extend_from_slice(&(self.rr_class as u16).to_be_bytes());
        vec.extend_from_slice(&self.ttl.to_be_bytes());
        vec.extend_from_slice(&self.rdlen.to_be_bytes());
        vec.extend_from_slice(&self.rdata);
        vec
    }
}

/// RRFormat holds multiple RRRecords for a given section (answer/authority/additional)
#[derive(Debug, Default)]
pub struct RRFormat {
    pub records: Vec<RRRecord>,
}

impl RRFormat {
    pub fn new(num: u16) -> Self {
        Self {
            records: Vec::with_capacity(num as usize),
        }
    }

    pub fn from_buffer(&mut self, buffer: &[u8]) -> usize {
        let mut pos = 0;
        for _ in 0..self.records.capacity() {
            let (record, next) = RRRecord::from_buffer(&buffer[pos..]);
            self.records.push(record);
            pos += next;
        }
        pos
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for record in &self.records {
            out.extend_from_slice(&record.to_buffer());
        }
        out
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
    AAAA = 28,
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
            28 => Ok(Self::AAAA),
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

    pub fn from_buffer(buffer: &[u8]) -> (Self, usize) {
        let mut labels = Labels::default();
        let mut start = 0_usize;

        loop {
            let num = buffer[start];

            if num == 0 {
                start += 1;
                break;
            }

            if num & 0xC0 == 0xC0 {
                start += 2;
                break;
            }

            let slice = &buffer[start + 1..start + 1 + num as usize];
            let string_value = str::from_utf8(slice).expect("Expected Str in the byte");
            labels.add_name(string_value.to_string());
            start += num as usize + 1;
        }

        (labels, start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rr_record() -> RRRecord {
        let mut name = Labels::default();
        name.add_name("www".to_string());
        name.add_name("google".to_string());
        name.add_name("com".to_string());

        RRRecord {
            name,
            rr_type: RRType::A,
            rr_class: RRClass::IN,
            ttl: 300,
            rdlen: 9,
            rdata: "127.0.0.1".to_string(),
        }
    }

    fn make_rr_format(num: u16) -> RRFormat {
        let mut fmt = RRFormat::new(num);
        for _ in 0..num {
            fmt.records.push(make_rr_record());
        }
        fmt
    }

    #[test]
    fn test_resolve_record_encode() {
        let rr = make_rr_record();
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
        let original = make_rr_record();
        let buffer = original.to_buffer();

        let (decoded, _) = RRRecord::from_buffer(&buffer);

        assert_eq!(decoded, original);
    }

    #[test]
    fn test_rr_format_multiple_records() {
        let original = make_rr_format(2);
        let buffer = original.to_buffer();

        let mut decoded = RRFormat::new(2);
        decoded.from_buffer(&buffer);

        assert_eq!(decoded.records.len(), 2);
        assert_eq!(decoded.records[0], original.records[0]);
        assert_eq!(decoded.records[1], original.records[1]);
    }

    #[test]
    fn test_labels_pointer_compression_skipped() {
        // pointer bytes 0xC0 0x0C should consume exactly 2 bytes and stop
        let buffer: &[u8] = &[0xC0, 0x0C, 0x00, 0x01];
        let (labels, consumed) = Labels::from_buffer(buffer);
        assert_eq!(consumed, 2);
        assert_eq!(labels.get_len(), 0); // no labels parsed, just pointer
    }
}
