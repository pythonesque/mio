#![allow(unused_variables)]

mod awakener;
mod io;
mod selector;
mod tcp;
mod udp;

pub use self::awakener::Awakener;
pub use self::io::Io;
pub use self::selector::{Events, Selector};
pub use self::tcp::TcpSocket;
pub use self::udp::UdpSocket;

// Re-export the needed APIs
mod api {
    pub use winapi::{
        HANDLE,
        INVALID_HANDLE_VALUE,
    };

    pub use kernel32::CreateIoCompletionPort;
}
