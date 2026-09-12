use colored::Colorize;
use rustls::pki_types::ServerName;
use tokio::{io::{AsyncWriteExt}, net::{TcpSocket}};
use tokio_rustls::{TlsConnector};
use std::{net::{Ipv4Addr, SocketAddrV4}, sync::Arc};
use tokio::process::Command;
//===============================================================
use crate::help_function::{progress, send_and_get, run_bash};
use crate::capute_isn::capute_isn;
use crate::all_ip::lookup_known_ip;
//use crate::fragmenting::FragmentingStream;
//===============================================================

pub async fn like_main()  -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    // Run bash script
    tokio::select! {
        status = run_bash() => {
            println!("Status = {status}\nLet's move on...");
        },
        _ = tokio::signal::ctrl_c() => {
            Command::new("./delete_file.sh")
                .status()
                .await
                .unwrap_or_else(|_| {
                    eprintln!("\n{}", "Error delete of file. Remove knows_ip.txt and result.txt please <3".black().on_red());
                    std::process::exit(0);
                });
        },
    }
    
    progress("[1/18] Loading sertificates...");
    //Create an empty list of certificates (simple terms)
    let mut root_cert = rustls::RootCertStore::empty();
    
    // This string load root-certificates from OS
    let native_cert = rustls_native_certs::load_native_certs()?;
    progress("[2/18] Sorting sertificates...");
    //get all certificates from certificates
    for cert in native_cert {
        root_cert.add(cert)?;
    }
    
    progress("[3/18] Creating Tls-client settings...");
    //create TLS-client settings
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_cert)        // We give OUR certificates (from "root_cert")
        .with_no_client_auth();                                                             //Don't use client's certificates

    progress("[4/18] Creating encryption tool...");
    //Create encryption tool
    let connector = TlsConnector::from(Arc::new(config));

    progress("[5/18] To seting domain...");
     let domain_str: &str = "youtube.com";    //just example

    //Get type "ServerName" and give owned to "domain"
    let domain = ServerName::try_from(domain_str)?.to_owned();

    progress("[6/18] Creating new socket...");
    // Create a new, not yet conected to anything socket (for Ipv4)
    let socket = TcpSocket::new_v4()?;

    progress("[7/18] Reserving random port...");
    // Ask OS reserve random port
    socket.bind("0.0.0.0:0".parse()?)?;

    progress("[8/18] Geting port...");
    // Get port
    let my_port = socket.local_addr()?.port();

    progress("[9/18] Finding ip in fresh list...");
    let ip_str = lookup_known_ip(domain_str).expect("This site is not in data");    //finding this domain in "knows_ip.txt" and return IP-addr
    let ip: Ipv4Addr = ip_str.parse().expect("Invalid IP format in file");                 // parsing string to Ipv4Addr type
    let server_some = SocketAddrV4::new(ip, 443);    

    let status = Command::new("iptables")
        .arg("-A")
        .arg("OUTPUT").arg("-p")
        .arg("tcp").arg("-d")
        .arg(ip_str)
        .arg("--dport").arg("443")
        .arg("-j").arg("NFQUEUE")
        .arg("--queue-num").arg("0")
        .status()
        .await.expect("Executing error");

    if status.success() {
        progress("[10/19] Successfully end of start NFQUEUE!");
    } else {
        panic!("NFQUEUE doesn't work!");
    }
    
    progress("[11/19] Runing additional stream...");

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    let handle = tokio::task::spawn_blocking(move || {
        progress("[12/19] Startint to sniff a packets");
        // Start to sniffing packets for find seq and ack
        capute_isn(server_some, my_port, tx)
    });
    rx.await.unwrap();
    progress("[13/19] We have found the SYN-ACK!");
    progress("[14/19] Connecting to server...");
    //just connect
    let stream = socket.connect(std::net::SocketAddr::V4(server_some)).await?;  

    progress("[15/19] Geting sequence and acknowlegement...");
    let (_sequence, _acknowlegement) = handle.await??;

    progress("[16/19] Geting ip...");
    let ip_adrr = stream.local_addr()?.ip();
    let my_ip = match ip_adrr {
        std::net::IpAddr::V4(addr) => addr,
        std::net::IpAddr::V6(_) => panic!("Ipv6 while is doesn't support!"),
    };

    let _my_ip = SocketAddrV4::new(my_ip, my_port);

    progress("[17/19] To waping our stream...");
    //Create a wrapper over stream
    //let stream = FragmentingStream::new(stream, my_ip, server_some, sequence, acknowlegement, 9);

    progress("[18/19] Runing Tls-handshake...");
    //Runing Tls HandShake
    let mut tls_stream = connector.connect(domain, stream).await?;

    progress("[19/19] To preparing the https request");
    let greet = b"GET / HTTP/1.1\r\n\
    Host: youtube.com\r\n\
    User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0\r\n\
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
    Accept-Language: en-US,en;q=0.5\r\n\
    Accept-Encoding: identity\r\n\
    Connection: close\r\n\r\n";
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