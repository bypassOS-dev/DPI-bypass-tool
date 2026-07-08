use tokio::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

// A wrapper around a standard TcpStream
pub struct FragmentingStream {
    inner: TcpStream,     //real TCP-socket
    fragments_sent: u32,  //counter of fragments already sent
    chunk_size: usize,    //The size of one piece in bytes
}
impl FragmentingStream {
    pub fn new(inner: TcpStream) -> Self {
        inner.set_nodelay(true).expect("We couldn't turn on Nodelay");  // Tell OS to disable Nagle's algorithm
        Self{inner, fragments_sent: 0, chunk_size: 0}                   // Return our "wrapper"
    }
}
impl AsyncRead for FragmentingStream {
    fn poll_read(
        self: Pin<&mut Self>,        // Needed to prevent object moving in memory
        cx: &mut Context<'_>,        //This "Context" is "waker" 
        buf: &mut ReadBuf<'_>        //Just buffer for data
    ) -> Poll<std::io::Result<()>>{
        let this = self.get_mut();
        Pin::new(&mut this.inner).poll_read(cx, buf)
    }
}
impl AsyncWrite for FragmentingStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
 
        if this.fragments_sent < 10 && buf.len() > 1 {           // If we sent less that 10 packet...
            if this.fragments_sent == 0 {                        // And if this is first packet...
                this.chunk_size = (buf.len() / 10).max(1);       // We get Size packet's piece 
            }                                                    // But if this number less that 10...
            let n = this.chunk_size.min(buf.len());       // We get size of piece (but if 'n' less that 'chunk_size' then just get remainder)
            this.fragments_sent += 1;                            
 
            Pin::new(&mut this.inner).poll_write(cx, &buf[..n])
        } else {
            Pin::new(&mut this.inner).poll_write(cx, buf)        // If this packet is simple then just write it
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