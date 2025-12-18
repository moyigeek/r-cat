use clap::Parser;
use r_cat::cli;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, UdpSocket};

#[tokio::test]
async fn tcp_echo_integration() -> anyhow::Result<()> {
    // Start a simple TCP echo server
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    let server = tokio::spawn(async move {
        let (mut socket, _peer) = listener.accept().await.expect("accept");
        let mut buf = [0u8; 1024];
        let n = socket.read(&mut buf).await.expect("read");
        socket.write_all(&buf[..n]).await.expect("write");
    });

    // Client: connect, send data, receive echo
    let mut client = tokio::net::TcpStream::connect(addr).await?;
    client.write_all(b"hello_tcp").await?;
    let mut res = vec![0u8; 9];
    client.read_exact(&mut res).await?;
    assert_eq!(&res, b"hello_tcp");

    // ensure server finished
    let _ = server.await;
    Ok(())
}

#[tokio::test]
async fn udp_echo_integration() -> anyhow::Result<()> {
    let server = UdpSocket::bind("127.0.0.1:0").await?;
    let server_addr = server.local_addr()?;

    let srv = tokio::spawn(async move {
        let mut buf = vec![0u8; 1500];
        let (n, peer) = server.recv_from(&mut buf).await.expect("recv");
        // echo back
        server.send_to(&buf[..n], peer).await.expect("send");
    });

    let client = UdpSocket::bind("127.0.0.1:0").await?;
    let msg = b"hello_udp";
    client.send_to(msg, server_addr).await?;
    let mut buf = vec![0u8; 1500];
    let (n, _peer) = client.recv_from(&mut buf).await?;
    assert_eq!(&buf[..n], msg);

    let _ = srv.await;
    Ok(())
}

#[test]
fn cli_parsing_integration() {
    let args = cli::Args::parse_from(&["r-cat", "-u", "-l", "-p", "1234"]);
    assert!(args.udp);
    assert!(args.listen);
    assert_eq!(args.source_port, Some(1234));

    let args2 = cli::Args::parse_from(&["r-cat", "example.com", "80"]);
    assert_eq!(args2.destination.as_deref(), Some("example.com"));
    assert_eq!(args2.port, Some(80));
}
