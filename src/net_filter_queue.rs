use nfq::{Queue, Verdict};
use pnet::packet::{Packet, ipv4::Ipv4Packet, tcp::TcpPacket};
use crate::fragmenting::sni_parser::find_sni;

pub fn _start_sniff() -> Result<(), Box<dyn std::error::Error>>{
    let mut queue = Queue::open()?;
    queue.bind(0)?;

    loop {
        let mut msg = queue.recv()?;
        let payload = msg.get_payload();

        if let Some(ip_packet) = Ipv4Packet::new(payload) {
            if let Some(tcp_packet) = TcpPacket::new(ip_packet.payload()) {
                let tcp_payload = tcp_packet.payload();
                println!(
                    "Packet: seq: {}, tcp payload lenght: {}",
                    tcp_packet.get_sequence(),
                    tcp_payload.len()
                );
                if tcp_payload.len() > 5 && tcp_payload[0] == 0x16 {
                    println!("This look like a TLS handshake packet!");
                }
                if let Some((split_pos, domain)) = find_sni(tcp_payload) {
                    println!("Found SNI: {} (split at {})", domain, split_pos)
                }
            };
        };

        println!("Got a packet! Size: {} bytes", payload.len());

        msg.set_verdict(Verdict::Accept);
        queue.verdict(msg)?;
    }
}
