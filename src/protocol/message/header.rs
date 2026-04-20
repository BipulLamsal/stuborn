use bit_vec::BitVec;

enum MessageHeader {
    /// A 16 bit identifier assigned by the program that
    /// generates any kind of query.  This identifier is copied
    /// the corresponding reply and can be used by the requester
    /// to match up replies to outstanding queries.
    ID(u16),
    /// one bit field that specifies whether this is query or message
    Qr(bool),
    /// 4 bit field that specifies kind of query in this message
    Opcode(Opcode),
    /// Authorative Answer : responding name server is an authority for the domain name
    /// in question section (1 bit)
    AA(u8),

    /// TrunCation - specifies that this message was truncated
    TC(u8),
    /// set in query and copied in the response, purse the query recursively
    RD(u8),
    /// must be zero in all queries and response
    Z(u8),
    /// Response Code : 4 bit field
    Rcode(ResponseCode),

    /// Number of entries in question section
    QdCount(u16),

    /// Number of entries in Answer section
    Ancount(u16),

    /// Number of name server resource records in the authority records
    Nscount(u16),

    /// Number of name server resource records in the additional section
    Arcount(u16),
}

#[repr(u8)]
enum Opcode {
    Query = 0,
    InverseQuery = 1,
    Status = 2,
}

#[repr(u8)]
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
