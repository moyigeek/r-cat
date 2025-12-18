use clap::Parser;
use std::time::Duration;

use r_cat::Args;
use r_cat::net::{tcp, udp};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI args (clap-derived)
    let args = Args::parse();

    // Convert optional timeout seconds into Duration
    let timeout = args.timeout.map(Duration::from_secs_f64);

    if args.listen {
        // Listen mode: need a port (positional `port` or `-p` `source_port`)
        let port = args
            .port
            .or(args.source_port)
            .ok_or_else(|| anyhow::anyhow!("listen mode requires a port (-p or positional)"))?;

        if args.udp {
            udp::listen(port, timeout, args.verbose).await?;
        } else {
            tcp::listen(port, timeout, args.verbose).await?;
        }
    } else {
        // Client mode: need destination host and port
        let host = args
            .destination
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("destination required in client mode"))?;
        let port = args
            .port
            .ok_or_else(|| anyhow::anyhow!("port required in client mode"))?;

        if args.udp {
            udp::client(host, port, timeout, args.verbose).await?;
        } else {
            tcp::client(host, port, timeout, args.verbose).await?;
        }
    }

    Ok(())
}
