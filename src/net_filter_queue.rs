use nfq::{Queue, Verdict};
use pnet::packet::{Packet, ipv4::Ipv4Packet, tcp::TcpPacket};
use crate::fragmenting::sni_parser::find_sni;
use rand::{Rng, thread_rng};
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
                    println!("Found SNI: {} (split at {})", domain, split_pos);

                    let mut rng = thread_rng();
                    let trash = rng.gen_range(10..=30);

                    let real_seq = tcp_packet.get_sequence();

                    let junk: Vec<u8> = vec![0x41; trash];
                    let mut packet1_payload = junk.clone();

                    packet1_payload.extend_from_slice(&tcp_payload[..split_pos]);

                    let packet1_seq = real_seq.wrapping_sub(trash as u32);

                    let packet2_payload =  &tcp_payload[split_pos..];

                    let packet2_seq = real_seq + split_pos as u32;

                    println!("Packet 1: seq={}, len={}", packet1_seq, packet1_payload.len());
                    println!("Packet 2: seq={}, len={}", packet2_seq, packet2_payload.len());

                }
            };
        };

        println!("Got a packet! Size: {} bytes", payload.len());

        msg.set_verdict(Verdict::Accept);
        queue.verdict(msg)?;
    }
}
