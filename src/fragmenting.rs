use tokio::net::TcpStream;
use std::net::SocketAddrV4;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
//=======================================================
mod sni_parser;
mod fake_ttl;
use sni_parser::find_sni;
use fake_ttl::send_fake_ttl;
//======================================================

// A wrapper around a standard TcpStream
pub struct FragmentingStream {
    inner: TcpStream,          //real TCP-socket
    first_write_done: bool,   // Just checking - is this packet the first one?
    my_ip: SocketAddrV4,
    server_ip: SocketAddrV4, 
    seq: u32,
    ack: u32,
    ttl: u8,
}
impl FragmentingStream {
    pub fn new(inner: TcpStream,
        my_ip: SocketAddrV4, 
        server_ip: SocketAddrV4, 
        seq: u32, 
        ack: u32, 
        ttl: u8) -> Self {

        inner.set_nodelay(true).expect("We couldn't turn off Nodelay");  // Tell OS to disable Nagle's algorithm
        Self { 
            inner, 
            first_write_done: false,
            my_ip,
            server_ip, 
            seq,
            ack,
            ttl,
        }      // Return our "wrapper"
    }                                                                  
}                                                                 
impl AsyncRead for FragmentingStream {
    fn poll_read(
        self: Pin<&mut Self>,          // Needed to prevent object moving in memory
        cx: &mut Context<'_>,         //This "Context" is "waker" 
        buf: &mut ReadBuf<'_>        //Just buffer for data
    ) -> Poll<std::io::Result<()>>{ 
        let this = self.get_mut();
        let result = Pin::new(&mut this.inner).poll_read(cx, buf);
        result
    }
}
impl AsyncWrite for FragmentingStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
 
        if !this.first_write_done && buf.len() > 1 {                                   //If this packet is the first one... 
                                                                                      //We are changing the state of "first_write_done" to true
                                                                                     //
            let split_at = find_sni(buf).unwrap_or(buf.len() / 2);  //Split domain or if hapens mistakes half of bufer
            let n = split_at.min(buf.len());                                //If somehow 1 piece more that all packet (It's error) then we              
                                                                                 //                                          just return half of packet
            match Pin::new(&mut this.inner).poll_write(cx, &buf[..n]) {// Sending that a piece
                Poll::Ready(Ok(n)) => {
                    this.first_write_done = true;  
                    //Create random text
                    let mut rng = thread_rng();
                    let random_num = rng.gen_range(10..30);

                    let random_text: Vec<u8> = (0..random_num)
                        .map(|_| rng.sample(Alphanumeric))
                        .collect();

                    if let Err(e) = send_fake_ttl(this.my_ip, this.server_ip, this.seq, this.ack, this.ttl, &random_text) {
                        eprintln!("[!!!]Falled to send fake packet: {e}");                        
                    };
                    return Poll::Ready(Ok(n));
                },
                Poll::Pending => {
                    return Poll::Pending;
                },
                Poll::Ready(Err(e)) => {
                    return Poll::Ready(Err(e));
                },
            }      
        } else {                                                              
            this.first_write_done = false; 
            Pin::new(&mut this.inner).poll_write(cx, buf)           // If this packet is simple then just write it
        }
    }
    fn poll_flush(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.inner).poll_flush(cx)
    }
    fn poll_shutdown(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.inner).poll_shutdown(cx)
    }
}
