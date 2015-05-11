#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{io, mem, ptr};

// Re-export
pub use winapi::{
    c_int,
    c_void,
    BOOL,
    GUID,
    DWORD,
    INVALID_HANDLE_VALUE,
    HANDLE,
    LPOVERLAPPED,
    LPDWORD,
    LPVOID,
    OVERLAPPED,
    ULONG_PTR,
    PULONG_PTR,
    WORD,
    PVOID,
    TRUE,
};
pub use libc::{
    atexit,
    bind,
    uintptr_t,
    in_addr,
    sa_family_t,
    sockaddr,
    sockaddr_in,
    sockaddr_storage,
    GetLastError,
    AF_INET,
    AF_INET6,
    SOCK_STREAM,
    SOCK_DGRAM,
};
pub use libc::consts::os::extra::{
    INVALID_SOCKET,
    WSAPROTOCOL_LEN,
};
pub use libc::types::os::arch::extra::{
    GROUP,
    WSAPROTOCOL_INFO,
    WSAPROTOCOLCHAIN,
};

/*
 *
 * ===== Types =====
 *
 */

pub type SOCKET = uintptr_t;
pub type LPWSAPROTOCOL_INFO = *mut WSAPROTOCOL_INFO;
pub type LPWSADATA = *mut WSADATA;
pub type LPWSAOVERLAPPED = LPOVERLAPPED;

pub const WSADESCRIPTION_LEN: usize = 256;
pub const WSASYS_STATUS_LEN: usize = 128;

pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE =
    extern "C" fn(dwErrorCode: DWORD,
                  dwNumberOfBytesTransfered: DWORD,
                  lpOverlapped: LPVOID);

pub type ConnectExFn =
    extern "C" fn(s: SOCKET,
                  name: *const sockaddr,
                  nameLen: c_int,
                  lpSendBuffer: PVOID,
                  dwSendDataLength: DWORD,
                  lpdwBytesSent: LPDWORD,
                  lpOverlapped: LPOVERLAPPED) -> BOOL;

#[repr(C)]
#[cfg(target_arch = "x86")]
pub struct WSADATA {
    pub wVersion: WORD,
    pub wHighVersion: WORD,
    pub szDescription: [u8; WSADESCRIPTION_LEN + 1],
    pub szSystemStatus: [u8; WSASYS_STATUS_LEN + 1],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut u8,
}

#[repr(C)]
#[cfg(target_arch = "x86_64")]
pub struct WSADATA {
    pub wVersion: WORD,
    pub wHighVersion: WORD,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut u8,
    pub szDescription: [u8; WSADESCRIPTION_LEN + 1],
    pub szSystemStatus: [u8; WSASYS_STATUS_LEN + 1],
}

/*
 *
 * ===== Constants =====
 *
 */

pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_UDP: c_int = 17;

pub const ERROR_IO_PENDING: DWORD = 0x3E5;
pub const WSA_FLAG_OVERLAPPED: DWORD = 0x01;
pub const WSA_FLAG_MULTIPOINT_C_ROOT: DWORD = 0x02;
pub const WSA_FLAG_MULTIPOINT_C_LEAF: DWORD = 0x04;
pub const WSA_FLAG_MULTIPOINT_D_ROOT: DWORD = 0x08;
pub const WSA_FLAG_MULTIPOINT_D_LEAF: DWORD = 0x10;
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: DWORD = 0x40;

pub const SIO_GET_EXTENSION_FUNCTION_POINTER: DWORD = 0xc8000006;

pub const WSAID_CONNECTEX: GUID = GUID {
    Data1: 0x25a207b9,
    Data2: 0xddf3,
    Data3: 0x4660,
    Data4: [0x8e, 0xe9, 0x76, 0xe5, 0x8c, 0x74, 0x06, 0x3e],
};

/*
 *
 * ===== FFI =====
 *
 */
extern "system" {
    pub fn CreateIoCompletionPort(FileHandle: HANDLE,
                                  ExistingCompletionPort: HANDLE,
                                  CompletionKey: ULONG_PTR,
                                  NumberOfConcurrentThreads: DWORD) -> HANDLE;

    pub fn GetQueuedCompletionStatus(CompletionPort: HANDLE,
                                     lpNumberOfBytes: LPDWORD,
                                     lpCompletionKey: PULONG_PTR,
                                     lpOverlapped: *mut LPOVERLAPPED,
                                     dwMilliseconds: DWORD) -> BOOL;

    fn WSASocketW(af: c_int,
                  kind: c_int,
                  protocol: c_int,
                  lpProtocolInfo: LPWSAPROTOCOL_INFO,
                  g: GROUP,
                  dwFlags: DWORD) -> SOCKET;

    fn WSAIoctl(s: SOCKET,
                dwIoControlCode: DWORD,
                lpvInBuffer: LPVOID,
                cbInBuffer: DWORD,
                lpvOutBuffer: LPVOID,
                cbOutBuffer: DWORD,
                lpcbBytesReturned: LPDWORD,
                lpOverlapped: LPWSAOVERLAPPED,
                lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> c_int;

    pub fn WSAStartup(wVersionRequested: WORD,
                      lpWSAData: LPWSADATA) -> c_int;

    pub fn WSACleanup() -> c_int;
}

pub fn WSASocket(af: c_int,
                 kind: c_int,
                 protocol: c_int,
                 lpProtocolInfo: LPWSAPROTOCOL_INFO,
                 g: GROUP,
                 dwFlags: DWORD) -> SOCKET {

    unsafe {
        WSASocketW(af, kind, protocol, lpProtocolInfo, g, dwFlags)
    }
}

/*
 *
 * ===== Initialize winsock =====
 *
 */

/// Checks whether the Windows socket interface has been started already, and
/// if not, starts it.
pub fn init_winsock() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    // Cleanup winsock resources
    extern "C" fn cleanup_winsock() {
        unsafe { WSACleanup(); }
    }

    START.call_once(|| {
        unsafe {
            let mut data: WSADATA = mem::zeroed();
            let ret = WSAStartup(0x202, &mut data); // Request version 2.2
            assert_eq!(ret, 0);

            atexit(cleanup_winsock);
        }
    });
}

unsafe fn get_extension_fn(socket: SOCKET, guid: &GUID) -> io::Result<*const c_void> {
    let mut ret: *const c_void = mem::uninitialized();
    let mut written = 0;

    let res = WSAIoctl(socket,
                       SIO_GET_EXTENSION_FUNCTION_POINTER,
                       mem::transmute(guid),
                       mem::size_of::<GUID>() as DWORD,
                       mem::transmute(&mut ret),
                       mem::size_of::<*const c_void>() as DWORD,
                       &mut written,
                       ptr::null_mut(),
                       mem::transmute(ptr::null::<c_void>()));

    if res != 0 {
        return Err(io::Error::last_os_error());
    }

    assert!(written as usize == mem::size_of::<*const c_void>());
    Ok(ret)
}

pub fn get_connect_ex_fn(socket: SOCKET) -> io::Result<ConnectExFn> {
    unsafe {
        get_extension_fn(socket, &WSAID_CONNECTEX)
            .map(|ptr| mem::transmute(ptr))
    }
}
