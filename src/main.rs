use rand::Rng;
use rustls::pki_types::ServerName;
use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}, net::{TcpSocket, TcpStream}};
use tokio_rustls::{TlsConnector, client::TlsStream};
use std::{net::{Ipv4Addr, SocketAddrV4}, sync::Arc};
use std::process::Command;
//===============================================================
mod fragmenting;
mod all_ip;
mod capute_isn;
use capute_isn::capute_isn;
use all_ip::lookup_known_ip;
use fragmenting::FragmentingStream;
//===============================================================
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let status = Command::new("./get_ip.sh")
        .status()
        .expect("[!!!]Script is fall");
    println!("Script has ended! \nStatus: {status} \nLet's move on...");
    //Create an empty list of certificates (simple terms)
    let mut root_cert = rustls::RootCertStore::empty();

    // This string load root-certificates from OS
    let native_cert = rustls_native_certs::load_native_certs()?;

    //get all certificates from certificates
    for cert in native_cert {
        root_cert.add(cert)?;
    }

    //create TLS-client settings
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_cert)        // We give OUR certificates (from "root_cert")
        .with_no_client_auth();                                                             //Don't use client's certificates

    //Create encryption tool
    let connector = TlsConnector::from(Arc::new(config));

    let domain_str = "youtube.com";     //just example

    //Get type "ServerName" and give owned to "domain"
    let domain = ServerName::try_from(domain_str)?.to_owned();

    let socket = TcpSocket::new_v4()?;

    socket.bind("0.0.0.0:0".parse()?)?;

    let my_port = socket.local_addr()?.port();

    let ip_str = lookup_known_ip(domain_str).expect("This site is not in data");    //finding this domain in "knows_ip.txt" and return IP-addr
    let ip: Ipv4Addr = ip_str.parse().expect("Invalid IP format in file");
    let server_some = SocketAddrV4::new(ip, my_port);

    let handle = tokio::task::spawn_blocking(move || {
        // Start to sniffing packets for find seq and ack
        capute_isn(server_some, my_port)
    });

    //just connect
    let stream = socket.connect(format!("{ip}:443").parse()?).await?;

    let (sequence, acknowlegement) = handle.await??;
    
    //Create a wrapper over stream
    let stream = FragmentingStream::new(stream);

    //Runing Tls HandShake
    let mut tls_stream = connector.connect(domain, stream).await?;

    let greet = b"GET / HTTP/1.1\r\nHost: www.youtube.com\r\nConnection: close\r\n\r\n";

    let mut buffer = [0u8; 1024];

    loop {
        tokio::select! {
            _ =  send_and_get(&mut tls_stream, greet, &mut buffer) => {
                println!("Your data was sent!");
            }
            _ = tokio::signal::ctrl_c() => {    //graceful shut down
                println!("\nShutting down...");
                tls_stream.shutdown().await?;
                break;
            }
        }
    }
    Ok(())
}
async fn send_and_get<S: AsyncRead + AsyncWrite + Unpin>(tls_stream: &mut TlsStream<S>, greet: &[u8], buffer: &mut [u8;1024]) {
    println!("Writing some...");
    let random = rand::thread_rng().gen_range(1..5000);
    tokio::time::sleep(tokio::time::Duration::from_millis(random)).await;

    if let Err(err) = tls_stream.write_all(greet).await {
        eprintln!("Write error: {err}");
    }
        
    let n = tls_stream.read(buffer).await.unwrap();
    let text = String::from_utf8_lossy(&buffer[..n]);

    println!("Text: {text}");
}
