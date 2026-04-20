// Resolve Record
struct RR_Format {
    /// Owner Name; name of the node to which record persists
    name: u16,
    /// 2 bytes of data containing one of the RR type codes
    rr_type: u16,
    /// 2 bytes of data containing one of the RR class codes
    rr_class: u16,
    /// 4 bytes of time data; zero value for volatile data (SOA record)  
    ttl: u32,
    /// length
    rdlen: u16,
    ///  variable length string that describes the resource
    rdata: u16,
}

// Qtype appears in the question part of a query
// Qtype are a superset of RRType, hence all RRTypes are valid Qtypes.
#[repr(u8)]
pub enum Qtype {
    Other(RRType),
    AXFR = 252,
    ALL = 255,
    // there are MAILB & MAILA which i am not implementing here
}

/// TYPE fields are used in resource records, they are sub field of the Qtype
#[repr(u8)]
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

/// CLASS fields appear in resource records.
#[repr(u8)]
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
