use std::{fmt, io, mem, ptr};
use std::cell::Cell;
use std::net::SocketAddr;
use sys::windows::{addr, api, Rt};

pub struct Socket {
    sock: api::SOCKET,
    overlapped: api::OVERLAPPED,
    connect_fn: Cell<Option<api::ConnectExFn>>,
}

impl Socket {
    pub fn new(sock: api::SOCKET) -> Socket {
        unsafe {
            Socket {
                sock: sock,
                overlapped: mem::zeroed(),
                connect_fn: Cell::new(None),
            }
        }
    }

    pub fn connect(&self, addr: &SocketAddr) -> io::Result<bool> {
        unsafe {
            // Get the connect function. This is gotten using WSAIoctl and the
            // value cached in the handle.
            let connect_fn = try!(self.connect_fn());
            let addr = addr::to_sockaddr(addr);

            // First bind
            let res = api::bind(self.sock,
                                mem::transmute(&ANY_IN),
                                mem::size_of::<api::sockaddr_in>() as api::c_int);

            if res != 0 {
                return Err(io::Error::last_os_error());
            }

            // Issue the connect
            let res = connect_fn(self.sock,
                                 &addr as *const api::sockaddr,
                                 mem::size_of::<api::sockaddr>() as api::c_int,
                                 ptr::null_mut(),
                                 0,
                                 ptr::null_mut(),
                                 mem::transmute(&self.overlapped));

            if res == api::TRUE {
                return Ok(true);
            }

            if api::GetLastError() == api::ERROR_IO_PENDING {
                return Ok(false);
            }

            return Err(io::Error::last_os_error());
        }
    }

    pub fn associate(&self, rt: &Rt) -> io::Result<()> {
        rt.associate_socket(self.sock)
    }

    fn connect_fn(&self) -> io::Result<api::ConnectExFn> {
        if let Some(f) = self.connect_fn.get() {
            return Ok(f);
        }

        api::get_connect_ex_fn(self.sock)
            .map(|f| {
                self.connect_fn.set(Some(f));
                f
            })
    }
}

impl fmt::Debug for Socket {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Socket {{ ... }}")
    }
}

static ANY_IN: api::sockaddr_in = api::sockaddr_in {
    sin_family: api::AF_INET as api::sa_family_t,
    sin_port: 0,
    sin_addr: api::in_addr { s_addr: 0 },
    sin_zero: [0, 0, 0, 0, 0, 0, 0, 0],
};
