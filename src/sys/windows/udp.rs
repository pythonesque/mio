use {io, Evented, Interest, IpAddr, PollOpt, Selector, Token};
use buf::{Buf, MutBuf};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct UdpSocket;

impl UdpSocket {
    pub fn v4() -> io::Result<UdpSocket> {
        unimplemented!();
    }

    /// Returns a new, unbound, non-blocking, IPv6 UDP socket
    pub fn v6() -> io::Result<UdpSocket> {
        unimplemented!();
    }

    pub fn bind(&self, addr: &SocketAddr) -> io::Result<()> {
        unimplemented!();
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    pub fn try_clone(&self) -> io::Result<UdpSocket> {
        unimplemented!();
    }

    pub fn send_to<B: Buf>(&self, buf: &mut B, target: &SocketAddr) -> io::Result<Option<()>> {
        unimplemented!();
    }

    pub fn recv_from<B: MutBuf>(&self, buf: &mut B) -> io::Result<Option<SocketAddr>> {
        unimplemented!();
    }

    pub fn set_broadcast(&self, on: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn set_multicast_loop(&self, on: bool) -> io::Result<()> {
        unimplemented!();
    }

    pub fn join_multicast(&self, multi: &IpAddr) -> io::Result<()> {
        unimplemented!();
    }

    pub fn leave_multicast(&self, multi: &IpAddr) -> io::Result<()> {
        unimplemented!();
    }

    pub fn set_multicast_time_to_live(&self, ttl: i32) -> io::Result<()> {
        unimplemented!();
    }
}

impl Evented for UdpSocket {
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
