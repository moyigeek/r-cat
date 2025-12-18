use std::time::Duration;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time;

/// TCP related helpers for r-cat.
///
/// This module exposes two async functions:
/// - `tcp_client` - connect to a remote TCP server and shuttle stdin <-> socket.
/// - `tcp_listen` - bind a TCP listener, accept one connection and shuttle stdin <-> socket.
///
/// These functions mirror the basic behavior previously implemented inline in main.
/// They return `anyhow::Result<()>` to simplify error propagation from the binary.
pub async fn client(
    host: &str,
    port: u16,
    timeout: Option<Duration>,
    verbose: bool,
) -> anyhow::Result<()> {
    let addr = format!("{}:{}", host, port);
    if verbose {
        eprintln!("Connecting to {}", addr);
    }

    let connect_fut = TcpStream::connect(addr);
    let stream = if let Some(dur) = timeout {
        match time::timeout(dur, connect_fut).await {
            Ok(Ok(s)) => s,
            Ok(Err(e)) => return Err(anyhow::anyhow!("connect error: {}", e)),
            Err(_) => return Err(anyhow::anyhow!("connect timed out after {:?}", dur)),
        }
    } else {
        connect_fut.await?
    };

    if verbose {
        eprintln!("Connected, starting IO copy");
    }

    // Split so we can read and write concurrently
    let (mut reader, mut writer) = stream.into_split();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // stdin -> socket
    let write_task = tokio::spawn(async move {
        let res = io::copy(&mut stdin, &mut writer).await;
        // attempt to shutdown the write half gracefully
        let _ = writer.shutdown().await;
        res
    });

    // socket -> stdout
    let read_task = tokio::spawn(async move {
        let res = io::copy(&mut reader, &mut stdout).await;
        let _ = stdout.flush().await;
        res
    });

    if let Some(dur) = timeout {
        match time::timeout(dur, async {
            let _ = tokio::join!(write_task, read_task);
        })
        .await
        {
            Ok(_) => {
                if verbose {
                    eprintln!("Session finished");
                }
            }
            Err(_) => {
                if verbose {
                    eprintln!("Session timed out after {:?}", dur);
                }
            }
        }
    } else {
        let _ = tokio::join!(write_task, read_task);
    }

    Ok(())
}

pub async fn listen(port: u16, timeout: Option<Duration>, verbose: bool) -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:{}", port);
    if verbose {
        eprintln!("Listening on {}", bind_addr);
    }
    let listener = TcpListener::bind(bind_addr).await?;
    let accept_fut = listener.accept();

    let (stream, peer) = if let Some(dur) = timeout {
        match time::timeout(dur, accept_fut).await {
            Ok(Ok((s, p))) => (s, p),
            Ok(Err(e)) => return Err(anyhow::anyhow!("accept failed: {}", e)),
            Err(_) => return Err(anyhow::anyhow!("accept timed out after {:?}", dur)),
        }
    } else {
        accept_fut.await?
    };

    if verbose {
        eprintln!("Accepted connection from {}", peer);
    }

    // shuttle IO same as client
    let (mut reader, mut writer) = stream.into_split();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let write_task = tokio::spawn(async move {
        let res = io::copy(&mut stdin, &mut writer).await;
        let _ = writer.shutdown().await;
        res
    });

    let read_task = tokio::spawn(async move {
        let res = io::copy(&mut reader, &mut stdout).await;
        let _ = stdout.flush().await;
        res
    });

    if let Some(dur) = timeout {
        match time::timeout(dur, async {
            let _ = tokio::join!(write_task, read_task);
        })
        .await
        {
            Ok(_) => {
                if verbose {
                    eprintln!("Connection finished");
                }
            }
            Err(_) => {
                if verbose {
                    eprintln!("Connection timed out after {:?}", dur);
                }
            }
        }
    } else {
        let _ = tokio::join!(write_task, read_task);
    }

    Ok(())
}
