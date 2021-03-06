use std::{
    io::{self},
    net::{TcpListener, TcpStream},
};

fn proxy(a: &TcpStream, b: &TcpStream) -> io::Result<()> {
    let ring = rio::new()?;
    let buf: &mut [u8] = &mut [0];
    loop {
        let read = ring.read_at_ordered(
            a,
            &buf,
            0,
            rio::Ordering::Link,
        )?;
        let write = ring.write_at(b, &buf, 0)?;
        read.wait()?;
        write.wait()?;
    }
}

fn main() -> io::Result<()> {
    let acceptor = TcpListener::bind("127.0.0.1:6666")?;

    for stream_res in acceptor.incoming() {
        let stream = stream_res?;
        proxy(&stream, &stream).unwrap_err();
    }

    Ok(())
}
