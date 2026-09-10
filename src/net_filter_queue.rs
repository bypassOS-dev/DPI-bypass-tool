use nfq::{Queue, Verdict};
use pnet::packet::{Packet, ipv4::Ipv4Packet, tcp::TcpPacket};
use crate::fragmenting::sni_parser::find_sni;
use rand::{Rng, thread_rng};

pub fn _start_sniff() -> Result<(), Box<dyn std::error::Error>>{
    // Create a queue:
    // The programm allocates memory and resourses for "queue" object
    // throught witch  will be sends or gets mesages 
    let mut queue = Queue::open()?;        
    queue.bind(0)?;

    loop {
        // "Catching" all packets
        // And get payload
        let mut msg = queue.recv()?;
        let payload = msg.get_payload();

        // It's main logics
        // First, we just check: Is this packet Ipv4?
        // If this packet is Ipv4 then we parse payload to Ipv4
        // And check later: Is payload into Ipv4 like TCP-protocol?
        // If these 2 check was successfully then this packet is valid
        if let Some(ip_packet) = Ipv4Packet::new(payload) {
            if let Some(tcp_packet) = TcpPacket::new(ip_packet.payload()) {
                // Get tcp-payload and just do simple output 
                let tcp_payload = tcp_packet.payload();
                println!(
                    "Packet: seq: {}, tcp payload lenght: {}",
                    tcp_packet.get_sequence(),
                    tcp_payload.len()
                );
                let mut checker = 0;

                // Do simple check: Is this packet TLS-handshake
                // Why that? 
                // Because Tls-payload MUST BE more that 5 bytes
                // And 0x16 - it is first bytes of tls-handshake
                if tcp_payload.len() > 5 && tcp_payload[0] == 0x16 {
                    println!("This look like a TLS handshake packet!");
                    checker += 1;
                }
                if checker ==  1 {
                    // "find-sni" - it is my function. 
                    // You can read her code!
                    // In a nutshell this function return 2 values:
                    // Split posicion and damain
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
                }
                
            };
        };

        println!("Got a packet! Size: {} bytes", payload.len());

        msg.set_verdict(Verdict::Accept);
        queue.verdict(msg)?;
    }
}
