use std::convert::TryInto;
use std::io;

use winapi::shared::minwindef::INT;
use winapi::um::{winnt::SHORT, winsock2};

pub const POLLRDNORM: SHORT = winsock2::POLLRDNORM;
pub const POLLRDBAND: SHORT = winsock2::POLLRDBAND;
pub const POLLIN: SHORT = winsock2::POLLIN;
pub const POLLPRI: SHORT = winsock2::POLLPRI;
pub const POLLWRNORM: SHORT = winsock2::POLLWRNORM;
pub const POLLOUT: SHORT = winsock2::POLLOUT;
pub const POLLWRBAND: SHORT = winsock2::POLLWRBAND;
pub const POLLERR: SHORT = winsock2::POLLERR;
pub const POLLHUP: SHORT = winsock2::POLLHUP;
pub const POLLNVAL: SHORT = winsock2::POLLNVAL;

pub use winsock2::WSAPOLLFD;

/// `wsa_poll` waits for one of a set of file descriptors to become ready to perform I/O.
///
/// This corresponds to calling [`WSAPoll`].
///
/// [`WSAPoll`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsapoll
pub fn wsa_poll(
    fd_array: &mut [WSAPOLLFD],
    timeout: INT,
) -> io::Result<usize> {
    unsafe {
        let length = fd_array.len().try_into().unwrap();
        let rc = winsock2::WSAPoll(fd_array.as_mut_ptr(), length, timeout);
        if rc < 0 {
            return Err(io::Error::last_os_error());
        };
        Ok(rc.try_into().unwrap())
    }
}
