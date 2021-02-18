use std::io;
use std::mem;

use winapi::um::{
    profileapi::QueryPerformanceFrequency, winnt::LARGE_INTEGER,
};

pub fn perf_counter_frequency() -> io::Result<u64> {
    unsafe {
        let mut frequency: LARGE_INTEGER = mem::zeroed();
        let rc = QueryPerformanceFrequency(&mut frequency);
        if rc == 0 {
            return Err(io::Error::last_os_error());
        };
        Ok(*frequency.QuadPart() as u64)
    }
}
