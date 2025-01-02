use crate::packet::header::TcpHeader;
use std::net::SocketAddr;

pub struct TcpSegment{
    header: TcpHeader,
    payload: Vec<u8>,
}

impl TcpSegment{
    pub fn new(source_addr: SocketAddr, dest_addr: SocketAddr, seq_num: u32, ack_num: u32, flags: u8, payload: Vec<u8>) -> Self{
        let mut header = TcpHeader::new(source_addr.port(), dest_addr.port(), seq_num, ack_num, flags);
        header.checksum = header.calculate_checksum(source_addr.ip(), dest_addr.ip(), &*payload);
        Self{header, payload}
    }

    pub fn to_bytes(&self) -> Vec<u8>{
        let mut packet = self.header.to_bytes();
        packet.extend_from_slice(&self.payload);
        packet
    }
}