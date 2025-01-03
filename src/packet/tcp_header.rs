use bytes::{BufMut, BytesMut};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::utils::sum_bytes_as_u32;

pub const TCP_FIN: u8 = 0x01;
pub const TCP_SYN: u8 = 0x02;
pub const TCP_RST: u8 = 0x04;
pub const TCP_PSH: u8 = 0x08;
pub const TCP_ACK: u8 = 0x10;
#[derive(Debug)]
pub struct TcpHeader{
    pub source_port: u16,
    pub dest_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub data_offset: u8,
    pub flags: u8,
    pub reserved: u8,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
}

impl TcpHeader{
    pub fn new(source_port: u16, dest_port: u16, seq_num: u32, ack_num: u32, flags: u8) -> Self{
        Self {
            source_port,
            dest_port,
            seq_num,
            ack_num,
            data_offset: 5,
            flags,
            reserved: 0,
            window_size: 65535,
            checksum: 0,
            urgent_ptr: 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8>{
        let mut bytes: BytesMut = BytesMut::with_capacity(20);

        bytes.put_u16(self.source_port);
        bytes.put_u16(self.dest_port);
        bytes.put_u32(self.seq_num);
        bytes.put_u32(self.ack_num);

        let mut drf = (self.data_offset as u16) << 12;
        drf |= (self.reserved as u16) << 9 ;
        drf |= self.flags as u16;
        bytes.put_u16(drf);

        bytes.put_u16(self.window_size);
        bytes.put_u16(self.checksum);
        bytes.put_u16(self.urgent_ptr);

        bytes.to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self>{
        println!("decaps : {:?}", bytes);
        if bytes.len() < 20 {
            return None;
        }

        let drf = u16::from_be_bytes(bytes[12..14].try_into().unwrap());
        let data_offset = ((drf >> 12) & 0b1111) as u8; // Top 4 bits
        let reserved = ((drf >> 9) & 0b111) as u8;      // Next 3 bits
        let flags = (drf & 0b1_1111_1111) as u8;        // Last 9 bits

        Some(TcpHeader{
            source_port: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
            dest_port: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
            seq_num: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            ack_num: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
            data_offset,
            flags,
            reserved,
            window_size: u16::from_be_bytes(bytes[14..16].try_into().unwrap()),
            checksum: u16::from_be_bytes(bytes[16..20].try_into().unwrap()),
            urgent_ptr: u16::from_be_bytes(bytes[20..24].try_into().unwrap()),
        })
    }



    pub fn calculate_checksum(&mut self, source_ip: IpAddr, dest_ip: IpAddr, payload: &[u8]) {
        match (source_ip, dest_ip) {
            (IpAddr::V4(source_ip), IpAddr::V4(dest_ip)) => {self.calculate_v4_checksum(source_ip, dest_ip, payload);},
            (IpAddr::V6(source_ip), IpAddr::V6(dest_ip)) => {self.calculate_v6_checksum(source_ip, dest_ip, payload);},
            _ => {panic!("Source and destination IP addresses must be of the same version");}
        };
    }

    fn calculate_v4_checksum(&mut self, source_ip: Ipv4Addr, dest_ip: Ipv4Addr, payload: &[u8]) {
        let mut data: u32 = 0;

        let header_as_bytes: Vec<u8> = self.to_bytes();
        data += u16::from_be_bytes(source_ip.octets()[0..2].try_into().unwrap()) as u32;
        data += u16::from_be_bytes(source_ip.octets()[2..4].try_into().unwrap()) as u32;
        data += u16::from_be_bytes(dest_ip.octets()[0..2].try_into().unwrap()) as u32;
        data += u16::from_be_bytes(dest_ip.octets()[2..4].try_into().unwrap()) as u32;


        data += 6u32;
        data += header_as_bytes.len() as u32 + payload.len() as u32;

        data += sum_bytes_as_u32(&header_as_bytes);
        data += sum_bytes_as_u32(&payload);

        while data >> 16 != 0 {
            data = (data & 0xFFFF) + (data >> 16);
        }

        self.checksum = !(data as u16)
    }

    fn calculate_v6_checksum(&self, source_ip: Ipv6Addr, dest_ip: Ipv6Addr, payload: &[u8]) -> u16{
        0
    }



}
