/*!
TODO.

Note that this crate is completely empty on non-Windows platforms.
*/

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
pub use win::*;

#[cfg(windows)]
mod win;
