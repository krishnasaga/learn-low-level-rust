use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    loop {
        let mut buf = [0; 4];
        let (_amt, _src) = socket.recv_from(&mut buf)?;
        for byte in std::str::from_utf8(&buf).unwrap().chars() {
            print!("{} ", byte);
        }
        if buf[0] == 0x00 {
            break;
        }
    }

    Ok(())
}
