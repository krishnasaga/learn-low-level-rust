use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8082")?;

    let remote_addr = "127.0.0.1:34254";

    socket.send_to(b"\xF0\x9F\x98\x8A", remote_addr)?;

    Ok(())
}
