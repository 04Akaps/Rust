use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
let host = "www.rustinaction.com:80";

// TCP를 사용할떄에는 http 요청과 다르게 포트번호를 입력해 주어야 한다.

    let mut connection = TcpStream::connect(host)?;

    connection.write_all(b"GET / HTTP/1.0")?;
    connection.write_all(b"/r/n")?;

    connection.write_all(b"Host: www.rustinaction.com")?;
    connection.write_all(b"/r/n/r/n")?;

    Ok(())
}
