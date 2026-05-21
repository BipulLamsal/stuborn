use std::net::UdpSocket;

use stubborn::protocol::message::{
    DNSpacket,
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
    question.add_name(String::from("www"));
    question.add_name(String::from("yahoo"));
    question.add_name(String::from("com"));
    let question_buffer = question.to_buffer();

    let mut req = Vec::from(header_buffer);
    req.extend(question_buffer);

    socket.connect(server).expect("connect function failed");

    let ret = socket.send(&req);

    if let Ok(bytes) = ret {
        println!("no of bytes sent: {}", bytes);
        let mut buf = [0; 512];

        let recv = socket.recv(&mut buf);

        if let Ok(r) = recv {
            println!("no of bytes received : {:?}", r);
        }

        let packet = DNSpacket::from_buffer(&buf);
        println!("{:#?}", packet);
    }
}
