use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let mut buf = [0; 4];
    let (_amt, _src) = socket.recv_from(&mut buf)?;
    for byte in std::str::from_utf8(&buf).unwrap().chars() {
        print!("{:X} ", byte as u32);
    }
    Ok(())
}
