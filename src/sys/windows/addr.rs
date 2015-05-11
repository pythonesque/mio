use std::mem;
use std::net::SocketAddr;
use libc::{
    in_addr,
    sa_family_t,
    sockaddr,
    sockaddr_in,
    AF_INET,
    AF_INET6,
};

pub fn to_sockaddr(addr: &SocketAddr) -> sockaddr {
    unsafe {
        match *addr {
            SocketAddr::V4(addr) => {
                let bytes = addr.ip().octets();

                mem::transmute(sockaddr_in {
                    sin_family: AF_INET as sa_family_t,
                    sin_port: addr.port().to_be(),
                    sin_addr: in_addr {
                        s_addr: (((bytes[0] as u32) << 24) |
                                 ((bytes[1] as u32) << 16) |
                                 ((bytes[2] as u32) <<  8) |
                                 ((bytes[3] as u32) <<  0)).to_be(),
                    },
                    sin_zero: mem::uninitialized(),
                })
            }
            SocketAddr::V6(addr) => {
                unimplemented!();
            }
        }
    }
}
