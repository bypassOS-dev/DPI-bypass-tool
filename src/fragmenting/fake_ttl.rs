use std::net::SocketAddrV4;
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::{self, MutableTcpPacket, TcpFlags}};
use pnet_transport::{transport_channel, TransportChannelType::Layer3};

pub fn send_fake_ttl(
    my_ip: SocketAddrV4,
    server_ip: SocketAddrV4,
    seq: u32,
    ack: u32,
    ttl: u8,
    random_text: &[u8]
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut tx, _) = transport_channel(2048, Layer3(IpNextHeaderProtocols::Tcp))?;

        //Calculate total lenght of TCP-segments in bytes:
       // 20 bytes: 
      // 4 bytes: sender's port (2 bytes) + recipient's port (2 bytes)
     // 10 bytes: Sequence Numbet (4 bytes) + Acknowledgement number (4 bytes) + flags and header lenght (2 bytes)
    // 6 bytes: Window Size (2 bytes) + Checksum (2 bytes) + Urgent Pointer (2 bytes) 
    let tcp_len = 20 + random_text.len();
    let mut tcp_buf = vec![0u8; tcp_len];
    let mut tcp_packet = MutableTcpPacket::new(&mut tcp_buf)
        .ok_or("Failed to create TCP packet buffer")?;

    tcp_packet.set_source(my_ip.port());
    tcp_packet.set_destination(server_ip.port());
    tcp_packet.set_sequence(seq);
    tcp_packet.set_acknowledgement(ack);
    tcp_packet.set_flags(TcpFlags::ACK | TcpFlags::PSH);
    tcp_packet.set_window(64240);
    tcp_packet.set_payload(random_text);

    let checksum = tcp::ipv4_checksum(&tcp_packet.to_immutable(), my_ip.ip(), server_ip.ip());
    tcp_packet.set_checksum(checksum);
    
    Ok(())
}