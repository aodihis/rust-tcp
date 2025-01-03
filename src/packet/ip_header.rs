use std::net::{IpAddr, Ipv4Addr};
use bytes::{BufMut, BytesMut};
use socket2::SockAddr;
use crate::utils::sum_bytes_as_u32;

#[derive(Debug)]
pub struct IpHeader{
    version: u8,
    ihl: u8,
    tos: u8,
    length: u16,
    id: u16,
    flags: u8,
    ttl: u8,
    protocol: u8,
    checksum: u16,
    src: IpAddr,
    dest: IpAddr,
}

impl IpHeader{
    pub fn new(source_addr: IpAddr, dest_addr: IpAddr, add_length: u16) -> Self {

        let mut version = 4;
        if source_addr.is_ipv6(){
            version = 6;
        }

        Self {
            version,
            ihl: 5,
            tos: 0,
            length: 20 + add_length,
            id: 0,
            flags: 0,
            ttl: 64,
            protocol: 6,
            checksum: 0,
            src: source_addr,
            dest: dest_addr,
        }

    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: BytesMut = BytesMut::with_capacity(20);
        bytes.put_u8((self.version << 4) | self.ihl);
        bytes.put_u8(self.tos);
        bytes.put_u16(self.length);
        bytes.put_u16(self.id);
        bytes.put_u16(((self.flags as u16) << 13) | (self.ttl as u16));
        bytes.put_u8(self.ttl);
        bytes.put_u8(self.protocol);
        bytes.put_u16(self.checksum);

        match self.src {
            IpAddr::V4(src) => {
                bytes.extend_from_slice(&src.octets());
            }
            IpAddr::V6(src) => {
                bytes.extend_from_slice(&src.octets());
            }
        };

        match self.dest {
            IpAddr::V4(dest) => {
                bytes.extend_from_slice(&dest.octets());
            }
            IpAddr::V6(dest) => {
                bytes.extend_from_slice(&dest.octets());
            }
        };
        bytes.to_vec()
    }

    pub fn calculate_checksum(&mut self) {
        let mut sum: u32 = 0;

        let header = self.to_bytes();

        sum += sum_bytes_as_u32(&header);
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        self.checksum = !(sum as u16)
    }
}