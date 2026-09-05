use std::net::SocketAddrV4;
use pnet::packet::{
    ip::{IpNextHeaderProtocols}, tcp::TcpFlags,
};
use pnet_transport::TransportChannelType::Layer4;
use pnet_transport::TransportProtocol::Ipv4;
use pnet_transport::{tcp_packet_iter, transport_channel};
use tokio::sync::oneshot::Sender;

pub fn capute_isn(server_some: SocketAddrV4, my_port: u16, tx: Sender<()>) -> Result<(u32, u32), Box<dyn std::error::Error + Send + Sync>> {
    let (_, mut rx) = transport_channel(2048, Layer4(Ipv4(IpNextHeaderProtocols::Tcp)))?;
    let mut iter = tcp_packet_iter(&mut rx);
    let _ = tx.send(());
    loop {
        let (packet, addr) = iter.next()?;

        let is_from_server = addr == std::net::IpAddr::V4(*server_some.ip());
        let is_right_port = packet.get_source() == server_some.port();
        let is_for_me = packet.get_destination() == my_port;
        let is_syn_ack = packet.get_flags() == (TcpFlags::SYN | TcpFlags::ACK);

        if is_from_server && is_right_port && is_for_me && is_syn_ack {
            let their_isn = packet.get_sequence();
            let our_seq = packet.get_acknowledgement();
            return Ok((our_seq, their_isn));
        }
    }
}