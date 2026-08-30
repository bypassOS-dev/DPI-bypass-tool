use std::net::SocketAddrV4;
use pnet::packet::{
    ip::{IpNextHeaderProtocols}, tcp::TcpFlags,
};
use pnet_transport::{TransportChannelType::Layer3, tcp_packet_iter, transport_channel};

pub fn capute_isn(server_some: SocketAddrV4, my_port: u16) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let (_tx, mut rx) = transport_channel(2048, Layer3(IpNextHeaderProtocols::Tcp))?;
    let mut iter = tcp_packet_iter(&mut rx);
    loop {
        let (packet, addr) = iter.next()?;

        let is_from_server = addr == std::net::IpAddr::V4(*server_some.ip());
        let is_right_port = packet.get_source() == server_some.port();
        let is_for_me = packet.get_destination() == my_port;
        let is_syn_ack = packet.get_flags() == (TcpFlags::ACK | TcpFlags::ACK);

        if is_from_server && is_right_port && is_for_me && is_syn_ack {
            let seq = packet.get_sequence();
            let ack = packet.get_acknowledgement();
            return Ok((seq, ack))
        }
    }
}
