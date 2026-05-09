use std::net::UdpSocket;

use stubborn::protocol::message::{
    header::{MessageHeader, MessageHeaderType},
    question::Question,
};

fn main() {
    let server = ("8.8.8.8", 53);
    let socket = UdpSocket::bind(("0.0.0.0", 43210)).expect("unable to bind the socket");
    let mut header = MessageHeader::default();
    header.add(MessageHeaderType::Id(69));
    header.add(MessageHeaderType::QdCount(1));
    header.add(MessageHeaderType::RD(true));
    let header_buffer = header.to_buffer();

    let mut question = Question::default();
    question.add_name(String::from("google.com"));
    let question_buffer = question.to_buffer();

    let mut req = Vec::from(header_buffer);
    req.extend(question_buffer);

    let ret = socket.send_to(&req, server);

    if let Ok(bytes) = ret {
        println!("no of bytes received: {}", bytes);
    }
}
