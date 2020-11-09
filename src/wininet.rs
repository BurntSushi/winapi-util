use widestring::U16CStr;

use winapi::shared::minwindef::FALSE;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::wininet::{
    HttpOpenRequestW, HttpSendRequestW, InternetCloseHandle, InternetConnectW,
    InternetOpenW, HINTERNET,
};

pub struct HInternetInner(HINTERNET);

impl HInternetInner {
    pub unsafe fn opt_from_raw(handle: HINTERNET) -> Option<Self> {
        if handle.is_null() {
            None
        } else {
            Some(HInternetInner(handle))
        }
    }
}

impl Drop for HInternetInner {
    fn drop(&mut self) {
        unsafe {
            assert_ne!(
                InternetCloseHandle(self.0),
                FALSE,
                "Got error code 0x{:8x} while closing HInternet {:p}.",
                GetLastError(),
                self.0
            );
        }
    }
}

fn opt_str_as_ptr(opt: Option<&U16CStr>) -> *const u16 {
    opt.map_or(std::ptr::null(), U16CStr::as_ptr)
}

pub struct HInternet(HInternetInner);

impl HInternet {
    pub fn open(
        agent: Option<&U16CStr>,
        access_type: u32,
        proxy: Option<&U16CStr>,
        proxy_bypass: Option<&U16CStr>,
        flags: u32,
    ) -> Option<Self> {
        unsafe {
            HInternetInner::opt_from_raw(InternetOpenW(
                opt_str_as_ptr(agent),
                access_type,
                opt_str_as_ptr(proxy),
                opt_str_as_ptr(proxy_bypass),
                flags,
            ))
            .map(HInternet)
        }
    }

    pub fn connect(
        &self,
        server_name: &U16CStr,
        server_port: u16,
        username: Option<&U16CStr>,
        password: Option<&U16CStr>,
        service: u32,
        flags: u32,
        context: usize,
    ) -> Option<HConnect> {
        unsafe {
            HInternetInner::opt_from_raw(InternetConnectW(
                (self.0).0,
                server_name.as_ptr(),
                server_port,
                opt_str_as_ptr(username),
                opt_str_as_ptr(password),
                service,
                flags,
                context,
            ))
            .map(HConnect)
        }
    }
}

pub struct HConnect(HInternetInner);

impl HConnect {
    pub fn open_request(
        &self,
        verb: Option<&U16CStr>,
        object_name: Option<&U16CStr>,
        version: Option<&U16CStr>,
        referrer: Option<&U16CStr>,
        accept_types: *const *const u16, // The `null-terminated` crate only works on nightly
        flags: u32,
        context: usize,
    ) -> Option<HRequest> {
        unsafe {
            HInternetInner::opt_from_raw(HttpOpenRequestW(
                (self.0).0,
                opt_str_as_ptr(verb),
                opt_str_as_ptr(object_name),
                opt_str_as_ptr(version),
                opt_str_as_ptr(referrer),
                accept_types as *mut _, // Probably a missing `const` in the C header?
                flags,
                context,
            ))
            .map(HRequest)
        }
    }
}

pub struct HRequest(HInternetInner);

impl HRequest {
    pub fn send(
        &self,
        headers: Option<&U16CStr>,
        optional: Option<&[u8]>,
    ) -> bool {
        unsafe {
            HttpSendRequestW(
                (self.0).0,
                opt_str_as_ptr(headers),
                headers.map_or(0, U16CStr::len) as u32,
                optional.map_or(std::ptr::null(), <[u8]>::as_ptr) as *mut _, // Probably a missing `const` in the C header?
                optional.map_or(0, <[u8]>::len) as u32,
            ) != FALSE
        }
    }
}
