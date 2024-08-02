use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8082")?;

    let remote_addr = "127.0.0.1:34254";
    for _i in [0; 1000] {
        socket.send_to(&[0xF0, 0x9F, 0x98, 0x8A], remote_addr)?;
    }

    socket.send_to(&[0x00], remote_addr)?;
    Ok(())
}
