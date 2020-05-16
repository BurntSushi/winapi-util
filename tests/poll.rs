#[cfg(windows)]
mod windows {
    use std::io::{Result, Write};
    use std::net::{TcpListener, TcpStream};
    use std::os::windows::io::{AsRawSocket, RawSocket};

    use winapi::um::winnt::SHORT;
    use winapi_util::socket::*;

    /// Get a pair of connected TcpStreams
    fn get_connection_pair() -> Result<(TcpStream, TcpStream)> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let stream1 = TcpStream::connect(listener.local_addr()?)?;
        let stream2 = listener.accept()?.0;

        Ok((stream1, stream2))
    }

    fn poll(socket: RawSocket, events: SHORT, revents: SHORT) -> Result<()> {
        let mut sockets = [WSAPOLLFD { fd: socket as _, events, revents: 0 }];
        let count = wsa_poll(&mut sockets, -1)?;
        assert_eq!(count, 1);
        assert_eq!(sockets[0].revents, revents);

        Ok(())
    }

    #[test]
    fn test_poll() -> Result<()> {
        let (mut stream1, stream2) = get_connection_pair()?;

        // Check that stream1 is writable
        poll(stream1.as_raw_socket(), POLLOUT, POLLOUT)?;

        // Write something to the stream
        stream1.write_all(b"1")?;

        // stream2 should now be readable and writable
        poll(stream2.as_raw_socket(), POLLIN | POLLOUT, POLLOUT | POLLRDNORM)?;

        Ok(())
    }
}
