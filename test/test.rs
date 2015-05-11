extern crate mio;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate tempdir;

pub use ports::localhost;

#[cfg(unix)]
mod test_battery;
#[cfg(unix)]
mod test_close_on_drop;
#[cfg(unix)]
mod test_echo_server;
#[cfg(unix)]
mod test_multicast;
#[cfg(unix)]
mod test_notify;
#[cfg(unix)]
mod test_register_deregister;
#[cfg(unix)]
mod test_timer;
#[cfg(unix)]
mod test_udp_socket;
#[cfg(unix)]
mod test_unix_echo_server;

mod ports {
    use std::net::SocketAddr;
    use std::str::FromStr;
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
    use std::sync::atomic::Ordering::SeqCst;

    // Helper for getting a unique port for the task run
    // TODO: Reuse ports to not spam the system
    static mut NEXT_PORT: AtomicUsize = ATOMIC_USIZE_INIT;
    const FIRST_PORT: usize = 18080;

    fn next_port() -> usize {
        unsafe {
            // If the atomic was never used, set it to the initial port
            NEXT_PORT.compare_and_swap(0, FIRST_PORT, SeqCst);

            // Get and increment the port list
            NEXT_PORT.fetch_add(1, SeqCst)
        }
    }

    pub fn localhost() -> SocketAddr {
        let s = format!("127.0.0.1:{}", next_port());
        FromStr::from_str(&s).unwrap()
    }
}

pub fn sleep_ms(ms: usize) {
    use std::thread;
    thread::sleep_ms(ms as u32);
}
