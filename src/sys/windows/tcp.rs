use {io, Evented, EventSet, PollOpt, Selector, Token};
use net::tcp::Shutdown;
use sys::windows::{api, Rt, Socket};
use std::io::{Read, Write};
use std::net::{SocketAddr};
use std::ptr;

#[derive(Debug)]
pub struct TcpSocket {
    socket: Box<Socket>,
}

impl TcpSocket {
    /// Returns a new, unbound, non-blocking, IPv4 socket
    pub fn v4() -> io::Result<TcpSocket> {
        use std::mem;
        TcpSocket::new(api::AF_INET)
    }

    /// Returns a new, unbound, non-blocking, IPv6 socket
    pub fn v6() -> io::Result<TcpSocket> {
        TcpSocket::new(api::AF_INET6)
    }

    fn new(family: api::c_int) -> io::Result<TcpSocket> {
        // Ensure that winsock is initialized
        api::init_winsock();

        let socket = api::WSASocket(
            family,
            api::SOCK_STREAM,
            api::IPPROTO_TCP,
            ptr::null_mut(),
            0,
            api::WSA_FLAG_OVERLAPPED);

        if socket == api::INVALID_SOCKET {
            return Err(io::Error::last_os_error());
        }

        Ok(TcpSocket { socket: Box::new(Socket::new(socket)) })
    }

    pub fn connect(&self, addr: &SocketAddr) -> io::Result<bool> {
        // TODO: don't initialize here
        let global = try!(Rt::global());

        try!(self.socket.associate(&global));
        try!(self.socket.connect(addr));

        Ok(true)
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

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
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

    pub fn take_socket_error(&self) -> io::Result<()> {
        unimplemented!();
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn set_keepalive(&self, seconds: Option<u32>) -> io::Result<()> {
        unimplemented!();
    }
}

impl Read for TcpSocket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!();
    }
}

impl Write for TcpSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!();
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!();
    }
}

impl Evented for TcpSocket {
    fn register(&self, selector: &mut Selector, token: Token, interest: EventSet, opts: PollOpt) -> io::Result<()> {
        unimplemented!();
    }

    fn reregister(&self, selector: &mut Selector, token: Token, interest: EventSet, opts: PollOpt) -> io::Result<()> {
        unimplemented!();
    }

    fn deregister(&self, selector: &mut Selector) -> io::Result<()> {
        unimplemented!();
    }
}
