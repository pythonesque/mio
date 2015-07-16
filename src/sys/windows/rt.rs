use io;
use sys::windows::api;
use std::{mem, ptr, sync, thread};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Rt {
    inner: &'static RtInner,
}

impl Rt {
    /// Returns a reference to the global `Rt`, lazily initializing one if
    /// needed.
    pub fn global() -> io::Result<Rt> {
        RtInner::global()
            .map(|inner| {

                Rt { inner: inner }
            })
    }

    pub fn associate_socket(&self, sock: api::SOCKET) -> io::Result<()> {
        self.inner.associate_socket(sock)
    }
}

pub struct Poll {
    inner: &'static RtInner,
}

impl Poll {
    pub fn global() -> io::Result<Poll> {
        RtInner::global()
            .map(|inner| {
                let refs = inner.refs.fetch_add(1, Ordering::Relaxed);

                if refs == 0 {
                    // TODO: Boot RT
                }

                Poll { inner: inner }
            })
    }

    fn poll(&self) -> io::Result<()> {
        self.inner.poll()
    }
}

impl Drop for Poll {
    fn drop(&mut self) {
        let refs = self.inner.refs.fetch_sub(1, Ordering::Relaxed);

        if refs == 1 {
            // TODO: Shutdown RT
        }
    }
}

static mut GLOBAL: Option<Result<RtInner, i32>> = None;

/// Manages the IOCP handle as well as the worker thread that performs the
/// required polling.
struct RtInner {
    refs: AtomicUsize,
    iocp: api::HANDLE,
}

impl RtInner {
    pub fn global() -> io::Result<&'static RtInner> {
        static INIT: sync::Once = sync::ONCE_INIT;

        INIT.call_once(|| {
            unsafe {
                GLOBAL = Some(RtInner::new().map_err(|e| e.raw_os_error().unwrap()));
            }
        });

        unsafe {
            match GLOBAL {
                Some(Ok(ref inner)) => Ok(inner),
                Some(Err(e)) => Err(io::Error::from_raw_os_error(e)),
                _ => panic!("should be set by now"),
            }
        }
    }

    /// Returns a new `Rt`
    fn new() -> io::Result<RtInner> {
        trace!("initializing a new RT");

        unsafe {
            let iocp = api::CreateIoCompletionPort(
                api::INVALID_HANDLE_VALUE,
                ptr::null_mut(),
                0,
                1);

            if iocp.is_null() {
                return Err(io::Error::last_os_error());
            }

            Ok(RtInner {
                iocp: iocp,
                refs: AtomicUsize::new(0),
            })
        }
    }

    /// Associates a socket with the `Rt`
    fn associate_socket(&self, sock: api::SOCKET) -> io::Result<()> {
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

    fn poll(&self) -> io::Result<()> {
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
