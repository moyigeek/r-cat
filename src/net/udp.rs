// r-cat/src/net/udp.rs
//! UDP helpers for r-cat.
//!
//! Exposes two async functions:
//! - `udp_client` - bind an ephemeral local UDP socket, send stdin as datagrams to a remote and
//!   print responses to stdout.
//! - `udp_listen` - bind to a local port, print incoming datagrams to stdout and allow sending
//!   stdin data to the last peer that sent a datagram.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::time;

pub async fn client(
    host: &str,
    port: u16,
    timeout: Option<Duration>,
    verbose: bool,
) -> anyhow::Result<()> {
    let remote = format!("{}:{}", host, port);
    let remote_addr: SocketAddr = remote
        .parse()
        .map_err(|e| anyhow::anyhow!("invalid remote address '{}': {}", remote, e))?;

    // Choose a wildcard bind address that matches the remote's IP family.
    let bind_addr = if remote_addr.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    };

    let socket = Arc::new(UdpSocket::bind(bind_addr).await?);

    if verbose {
        eprintln!(
            "udp: bound to {}, sending to {}",
            socket.local_addr()?,
            remote_addr
        );
    }

    // Send task: read stdin and send datagrams to remote.
    let send_socket = socket.clone();
    let send_task = tokio::spawn(async move {
        let mut stdin = io::stdin();
        let mut buf = vec![0u8; 8192];
        loop {
            match stdin.read(&mut buf).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    // best-effort send; ignore result but break on fatal error would also be acceptable
                    let _ = send_socket.send_to(&buf[..n], remote_addr).await;
                }
                Err(_) => break,
            }
        }
    });

    // Receive task: print incoming datagrams to stdout.
    let recv_socket = socket.clone();
    let recv_task = tokio::spawn(async move {
        let mut buf = vec![0u8; 65536];
        let mut stdout = io::stdout();
        loop {
            match recv_socket.recv_from(&mut buf).await {
                Ok((n, _src)) => {
                    let _ = stdout.write_all(&buf[..n]).await;
                    let _ = stdout.flush().await;
                }
                Err(_) => break,
            }
        }
    });

    // Wait for both tasks, optionally applying a timeout to the whole session.
    if let Some(dur) = timeout {
        match time::timeout(dur, async {
            let _ = tokio::join!(send_task, recv_task);
        })
        .await
        {
            Ok(_) => {
                if verbose {
                    eprintln!("udp: session finished");
                }
            }
            Err(_) => {
                if verbose {
                    eprintln!("udp: session timed out after {:?}", dur);
                }
            }
        }
    } else {
        let _ = tokio::join!(send_task, recv_task);
    }

    Ok(())
}

pub async fn listen(port: u16, timeout: Option<Duration>, verbose: bool) -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:{}", port);
    if verbose {
        eprintln!("udp: listening on {}", bind_addr);
    }

    let socket = Arc::new(UdpSocket::bind(&bind_addr).await?);
    if verbose {
        eprintln!("udp: bound to {}", socket.local_addr()?);
    }

    // Track the last peer we heard from so stdin can send to it.
    let last_peer: Arc<Mutex<Option<SocketAddr>>> = Arc::new(Mutex::new(None));

    // Receive task: record peer and print incoming payloads to stdout.
    let recv_peer = last_peer.clone();
    let recv_socket = socket.clone();
    let recv_task = tokio::spawn(async move {
        let mut buf = vec![0u8; 65536];
        let mut stdout = io::stdout();
        loop {
            match recv_socket.recv_from(&mut buf).await {
                Ok((n, src)) => {
                    // record peer
                    {
                        let mut guard = recv_peer.lock().await;
                        *guard = Some(src);
                    }
                    let _ = stdout.write_all(&buf[..n]).await;
                    let _ = stdout.flush().await;
                }
                Err(_) => break,
            }
        }
    });

    // Send task: read stdin and send to last seen peer (if any).
    let send_peer = last_peer.clone();
    let send_socket = socket.clone();
    let send_task = tokio::spawn(async move {
        let mut stdin = io::stdin();
        let mut buf = vec![0u8; 8192];
        loop {
            match stdin.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    // get current peer snapshot
                    let opt_peer = { send_peer.lock().await.clone() };
                    if let Some(peer) = opt_peer {
                        let _ = send_socket.send_to(&buf[..n], peer).await;
                    } else {
                        // no peer yet; drop data
                    }
                }
                Err(_) => break,
            }
        }
    });

    if let Some(dur) = timeout {
        match time::timeout(dur, async {
            let _ = tokio::join!(recv_task, send_task);
        })
        .await
        {
            Ok(_) => {
                if verbose {
                    eprintln!("udp: listen finished");
                }
            }
            Err(_) => {
                if verbose {
                    eprintln!("udp: listen timed out after {:?}", dur);
                }
            }
        }
    } else {
        let _ = tokio::join!(recv_task, send_task);
    }

    Ok(())
}
