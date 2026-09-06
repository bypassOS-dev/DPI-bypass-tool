use rand::Rng;
use colored::Colorize;
use tokio::process::Command;
use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}};
use tokio_rustls::{client::TlsStream};
use std::{io::Write, process::{ExitStatus}};

pub async fn send_and_get<S: AsyncRead + AsyncWrite + Unpin>(tls_stream: &mut TlsStream<S>, greet: &[u8], buffer: &mut [u8;1024]) {
    println!("Writing some...");
    let random = rand::thread_rng().gen_range(1..5000);
    tokio::time::sleep(tokio::time::Duration::from_millis(random)).await;

    if let Err(err) = tls_stream.write_all(greet).await {
        eprintln!("Write error: {err}");
    }
        
    let n = tls_stream.read(buffer).await.unwrap();
    let text = String::from_utf8_lossy(&buffer[..n]);

    println!("Text: \x1b[4m{text}\x1b[0m");
}

pub async fn run_bash() -> ExitStatus{
    let status = Command::new("./get_ip.sh")
        .status()
        .await
        .unwrap_or_else(|_| {
            eprintln!("{}", "[!!!]Script is fall. I'm sorry but maybe someone change a bash script.".black().on_red());
            std::process::exit(0);
        });
    status
}

pub fn progress(step: &str) {
    print!("\r\x1b[2K{}", step);
    std::io::stdout().flush().unwrap();
}