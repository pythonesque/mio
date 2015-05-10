use io;
use event::IoEvent;
use std::ptr;
use sys::windows::api::*;

#[derive(Debug)]
pub struct Selector {
    iocp: HANDLE,
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        unsafe {
            let iocp = CreateIoCompletionPort(
                INVALID_HANDLE_VALUE,   // FileHandle
                ptr::null_mut(),        // Existing completion port
                0,                      // Completion key for this port
                1);

            if iocp.is_null() {
                return Err(io::Error::last_os_error());
            }

            Ok(Selector { iocp: iocp })
        }
    }

    pub fn select(&mut self, evts: &mut Events, timeout_ms: usize) -> io::Result<()> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct Events;

impl Events {
    pub fn new() -> Events {
        unimplemented!();
    }

    pub fn len(&self) -> usize {
        unimplemented!();
    }

    pub fn get(&self, idx: usize) -> IoEvent {
        unimplemented!();
    }
}
