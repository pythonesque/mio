use io;
use sys::windows::api;
use std::{mem, ptr, sync};

#[derive(Debug)]
pub struct Rt {
    iocp: api::HANDLE,
}

static mut GLOBAL: Option<Result<Rt, i32>> = None;

impl Rt {
    pub fn global() -> io::Result<&'static Rt> {
        static INIT: sync::Once = sync::ONCE_INIT;

        INIT.call_once(|| {
            unsafe {
                GLOBAL = Some(Rt::new().map_err(|e| e.raw_os_error().unwrap()));
            }
        });

        unsafe {
            match GLOBAL {
                Some(Ok(ref rt)) => Ok(rt),
                Some(Err(e)) => {
                    Err(io::Error::from_raw_os_error(e))
                }
                _ => panic!("should be set by now"),
            }
        }
    }

    pub fn new() -> io::Result<Rt> {
        unsafe {
            let iocp = api::CreateIoCompletionPort(
                api::INVALID_HANDLE_VALUE,
                ptr::null_mut(),
                0,
                1);

            if iocp.is_null() {
                return Err(io::Error::last_os_error());
            }

            Ok(Rt { iocp: iocp })
        }
    }

    pub fn associate_socket(&self, sock: api::SOCKET) -> io::Result<()> {
        let res = unsafe {
            api::CreateIoCompletionPort(
                sock as api::HANDLE,
                self.iocp,
                0,
                0)
        };

        if res != self.iocp {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn poll(&self) -> io::Result<()> {
        unsafe {
            let mut bytes: api::DWORD = mem::uninitialized();
            let mut key: api::ULONG_PTR = mem::uninitialized();
            let mut overlapped: *mut api::OVERLAPPED = mem::uninitialized();

            let res = api::GetQueuedCompletionStatus(self.iocp,
                                                     &mut bytes as api::LPDWORD,
                                                     &mut key as api::PULONG_PTR,
                                                     &mut overlapped as *mut api::LPOVERLAPPED,
                                                     10000);

            assert!(res == api::TRUE);
        }

        Ok(())
    }
}

unsafe impl Send for Rt { }
unsafe impl Sync for Rt { }
