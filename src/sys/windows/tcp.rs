use {io, Evented, Interest, PollOpt, Selector, Token, TryRead, TryWrite};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct TcpSocket;

impl TcpSocket {
    /// Returns a new, unbound, non-blocking, IPv4 socket
    pub fn v4() -> io::Result<TcpSocket> {
        unimplemented!();
    }

    /// Returns a new, unbound, non-blocking, IPv6 socket
    pub fn v6() -> io::Result<TcpSocket> {
        unimplemented!();
    }

    pub fn connect(&self, addr: &SocketAddr) -> io::Result<bool> {
        unimplemented!();
    }

    pub fn bind(&self, addr: &SocketAddr) -> io::Result<()> {
        unimplemented!();
    }

    pub fn listen(&self, backlog: usize) -> io::Result<()> {
        unimplemented!();
    }

    pub fn accept(&self) -> io::Result<Option<TcpSocket>> {
        unimplemented!();
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    pub fn try_clone(&self) -> io::Result<TcpSocket> {
        unimplemented!();
    }

    /*
     *
     * ===== Socket Options =====
     *
     */

    pub fn set_reuseaddr(&self, val: bool) -> io::Result<()> {
        unimplemented!();
    }
}

impl TryRead for TcpSocket {
    fn read_slice(&mut self, buf: &mut [u8]) -> io::Result<Option<usize>> {
        unimplemented!();
    }
}

impl TryWrite for TcpSocket {
    fn write_slice(&mut self, buf: &[u8]) -> io::Result<Option<usize>> {
        unimplemented!();
    }
}

impl Evented for TcpSocket {
    fn register(&self, selector: &mut Selector, token: Token, interest: Interest, opts: PollOpt) -> io::Result<()> {
        unimplemented!();
    }

    fn reregister(&self, selector: &mut Selector, token: Token, interest: Interest, opts: PollOpt) -> io::Result<()> {
        unimplemented!();
    }

    fn deregister(&self, selector: &mut Selector) -> io::Result<()> {
        unimplemented!();
    }
}
