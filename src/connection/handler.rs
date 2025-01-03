use crate::connection::state::TcpState;
use crate::error::{Result, TcpError};
use crate::packet::tcp_header::{TcpHeader, TCP_ACK, TCP_PSH, TCP_SYN};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::time::Duration;
use crate::packet::ip_header::IpHeader;

#[derive(Debug)]
pub struct TcpConnection {
    state: TcpState,
    local_addr: SocketAddr,
    remote_addr: SocketAddr,
    seq_num: u32,
    ack_num: u32,
    socket: Socket,
    window_size: u32,
}

impl TcpConnection {
    pub fn connect<A: ToSocketAddrs>(remote_addr: A) -> Result<Self> {
        // let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
        let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(255)))?;
        // socket.set_nonblocking(true)?;
        socket.set_header_included_v4(true)?;
        let addr = remote_addr.to_socket_addrs()?.next().unwrap();

        let mut connection: TcpConnection = TcpConnection{
            state: TcpState::Closed,
            local_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 12345),
            remote_addr: addr,
            seq_num: rand::random::<u32>(),
            ack_num: 0,
            socket,
            window_size: 65535,
        };
        println!("TCP Connection start");
        connection.perform_handshake()?;
        Ok(connection)
    }

    fn send_data(&mut self, data: &[u8]) -> Result<usize> {
        let mut header = TcpHeader::new(self.local_addr.port(), self.remote_addr.port(), self.seq_num, self.ack_num, TCP_PSH | TCP_ACK);
        let mut packet = header.to_bytes();
        packet.extend_from_slice(data);

        let bytes_sent = self.socket.send_to(&packet, &SockAddr::from(self.remote_addr))?;
        self.seq_num += self.seq_num.wrapping_add(data.len() as u32);
        Ok(bytes_sent - header.to_bytes().len())
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize> {
        if self.state != TcpState::Established {
            return Err(TcpError::ConnectionError("Connection not established".to_string()));
        }
        self.send_data(data)
    }


    fn perform_handshake(&mut self) -> Result<()> {
        self.send_sync()?;
        self.state = TcpState::SynSent;

        let timeout = Duration::from_secs(5);
        let start = std::time::Instant::now();
        while start.elapsed() < timeout {
            match self.receive_packet() {
                Ok(packet) => {
                    println!("TCP Connection received: {:?}", packet);
                    if let Some(header) = TcpHeader::from_bytes(&packet[20..40]) {
                        if self.handle_handshake_packet(header)? {
                            return Ok(());
                        }
                    }
                }
                Err(TcpError::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(10));
                    continue;
                }
                Err(e) => {return Err(e)}
            }
        }

        Err(TcpError::ConnectionError("Timeout".to_string()))
    }

    fn handle_handshake_packet(&mut self, header: TcpHeader) -> Result<bool> {
        match self.state {
            TcpState::SynSent => {
                if header.flags == (TCP_ACK | TCP_SYN) {
                    self.ack_num = header.seq_num + 1;
                    self.send_ack().expect("Please check the header");
                    self.state = TcpState::Established;
                    return Ok(true);
                }
            }
            _ => {}
        }

        Ok(false)
    }

    fn send_sync(&mut self) -> Result<()> {
        let mut tcp_header = TcpHeader::new(self.local_addr.port(), self.remote_addr.port(), self.seq_num, self.ack_num, TCP_SYN);
        let mut ip_header = IpHeader::new(self.local_addr.ip(), self.remote_addr.ip(), 20);
        tcp_header.calculate_checksum(self.local_addr.ip(), self.remote_addr.ip(), &[]);
        ip_header.calculate_checksum();
        let mut packet = Vec::new();
        packet.extend_from_slice(&ip_header.to_bytes());
        packet.extend_from_slice(&tcp_header.to_bytes());
        println!("TCP Sync send {:?}",packet );
        let size = self.socket.send_to(&packet, &SockAddr::from(self.remote_addr))?;
        println!("{}", size);
        Ok(())
    }

    fn send_ack(&mut self) -> Result<()> {
        let header = TcpHeader::new(self.local_addr.port(), self.remote_addr.port(), self.seq_num, self.ack_num, TCP_ACK);
        let packet = header.to_bytes();
        self.socket.send_to(&packet, &SockAddr::from(self.local_addr))?;
        Ok(())

    }

    fn receive_packet(&mut self) -> Result<Vec<u8>> {
        let mut buffer:Vec<u8> = Vec::with_capacity(1500);
        let (size, _) = self.socket.recv_from(buffer.spare_capacity_mut())?;
        unsafe { buffer.set_len(size) };
        Ok(buffer)
    }
}