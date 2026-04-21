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
struct Question {
    // this can be of variable size a domain name represented as a sequence of labels, where each label consists of a length octet followed by that number of octets.
    qname: String,
    q_type: RRType,
    class: RRClass,
}
