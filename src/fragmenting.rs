use tokio::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
pub struct FragmentingStream {
    inner: TcpStream,
    fragments_sent: u32,
    chunk_size: usize,
}
impl FragmentingStream {
    pub fn new(inner: TcpStream) -> Self {
        inner.set_nodelay(true).expect("We couldn't turn on Nodelay");
        Self{inner, fragments_sent: 0, chunk_size: 0}
    }
}
impl AsyncRead for FragmentingStream {
    fn poll_read(
        self: Pin<&mut Self>,        //It's need because rust look for move of object
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
 
        if this.fragments_sent < 10 && buf.len() > 1 {
            if this.fragments_sent == 0 {
                this.chunk_size = (buf.len() / 10).max(1);
            }
            let n = this.chunk_size.min(buf.len());
            this.fragments_sent += 1;
 
            Pin::new(&mut this.inner).poll_write(cx, &buf[..n])
        } else {
            Pin::new(&mut this.inner).poll_write(cx, buf)
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