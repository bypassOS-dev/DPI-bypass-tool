use std::net::SocketAddrV4;

pub fn send_fake_ttl(
    my_ip: SocketAddrV4,
    server_ip: SocketAddrV4,
    seq: u32,
    ack: u32,
    ttl: u8,
    random_text: &[u8]
) -> Result<(), Box<dyn std::error::Error>> {
    
    Ok(())
}