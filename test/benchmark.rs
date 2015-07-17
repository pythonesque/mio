use std::mem;
use mio::net::{AddressFamily, Inet, Inet6, SockAddr, InetAddr, IPv4Addr, SocketType, Dgram, Stream};
use std::io::net::ip::IpAddr;
use native::NativeTaskBuilder;
use std::task::TaskBuilder;
use mio::os::{from_sockaddr};
use time;
use std::vec::*;
use std::io::timer;

mod nix {
    pub use nix::c_int;
    pub use nix::fcntl::{Fd, O_NONBLOCK, O_CLOEXEC};
    pub use nix::errno::{EWOULDBLOCK, EINPROGRESS};
    pub use nix::sys::socket::*;
    pub use nix::unistd::*;
    pub use nix::sys::epoll::*;
}

fn timed<F: Fn()>(label: &str, f: F) {
    let start = time::precise_time_s();
    f();
    let end = time::precise_time_s();
    println!("  {}: {}", label, end - start);
}

fn init(saddr: &str) -> (nix::Fd, nix::Fd) {
    let optval = 1isize;
    let addr = SockAddr::parse(saddr.as_slice()).expect("could not parse InetAddr");
    let srvfd = nix::socket(nix::AF_INET, nix::SOCK_STREAM, nix::SOCK_CLOEXEC).unwrap();
    nix::setsockopt(srvfd, nix::SOL_SOCKET, nix::SO_REUSEADDR, &optval).unwrap();
    nix::bind(srvfd, &from_sockaddr(&addr)).unwrap();
    nix::listen(srvfd, 256usize).unwrap();

    let fd = nix::socket(nix::AF_INET, nix::SOCK_STREAM, nix::SOCK_CLOEXEC | nix::SOCK_NONBLOCK).unwrap();
    let res = nix::connect(fd, &from_sockaddr(&addr));
    println!("connecting : {} - {}", res, time::precise_time_s());

    let clifd = nix::accept4(srvfd, nix::SOCK_CLOEXEC | nix::SOCK_NONBLOCK).unwrap();
    println!("accepted : {} - {}", clifd, time::precise_time_s());

    (clifd, srvfd)
}

#[test]
fn read_bench() {
    let (clifd, srvfd) = init("10.10.1.5:11111");
    let mut buf = Vec::with_capacity(1600);
    unsafe { buf.set_len(1600); }
    timed("read", || {
        let mut i = 0usize;
        while i < 10000000 {
            let res = nix::read(clifd, buf.as_mut_slice());
            assert_eq!(res.unwrap_err().kind, nix::EWOULDBLOCK);
            i = i + 1;
        }
    });
}

#[test]
fn epollctl_bench() {
    let (clifd, srvfd) = init("10.10.1.5:22222");

    let epfd = nix::epoll_create().unwrap();
    let info = nix::EpollEvent { events: nix::EPOLLIN | nix::EPOLLONESHOT | nix::EPOLLET,
                                 data: 0u64 };

    nix::epoll_ctl(epfd, nix::EpollCtlAdd, clifd, &info);

    timed("epoll_ctl", || {
        let mut i = 0usize;
        while i < 10000000 {
            nix::epoll_ctl(epfd, nix::EpollCtlMod, clifd, &info);
            i = i + 1;
        }
    });

}
