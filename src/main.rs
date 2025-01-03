use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use socket2::{Domain, Protocol, Socket, Type, SockAddr};
use tcp::packet::tcp_header::{TcpHeader, TCP_SYN};
use crate::connection::handler::TcpConnection;
use crate::packet::ip_header::IpHeader;

mod packet;
mod connection;
mod error;
mod utils;

fn main() {
    //
    // let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(255))).unwrap();
    // // let mut socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
    // socket.set_nonblocking(true).unwrap();
    // socket.set_header_included_v4(true).unwrap();
    // let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    //
    // let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 23);
    //
    //
    // let mut ip_header = IpHeader::new(addr.ip(), remote_addr.ip(), 20);
    // ip_header.calculate_checksum();
    // let mut header  = TcpHeader::new(12345, 23,0, 0, TCP_SYN);
    // header.calculate_checksum(addr.ip(), remote_addr.ip(), &[]);
    // let mut packet = vec![];
    // packet.extend_from_slice(&ip_header.to_bytes());
    // packet.extend_from_slice(&header.to_bytes());
    // println!("{:?}", header);
    // println!("{:?}", ip_header);
    // println!("{:?}", packet);
    // let res = socket.send_to(&packet, &SockAddr::from(remote_addr));
    // match res {
    //     Ok(size) => println!("{}", size),
    //     Err(err) => println!("errrr {}", err),
    // }


    let connection = TcpConnection::connect("127.0.0.1:23");

    match connection {
        Ok(stream) => {
            println!("{:?}", stream);
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    // TcpStream::connect("127.0.0.1:8080").unwrap();
}
