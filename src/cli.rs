use clap::Parser;

/// CLI args with a full set of OpenBSD-netcat-like flags.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    disable_version_flag = true, // 把 `-V` 留给 rtable
)]
pub struct Args {
    /* ---------- 地址族 ---------- */
    /// Force IPv4
    #[arg(short = '4', long = "ipv4")]
    pub ipv4: bool,

    /// Force IPv6
    #[arg(short = '6', long = "ipv6")]
    pub ipv6: bool,

    /* ---------- 传输层协议 ---------- */
    /// Use UDP instead of TCP
    #[arg(short, long)]
    pub udp: bool,

    /// Use Unix-domain socket
    #[arg(short = 'U')]
    pub unix: bool,

    /* ---------- 模式/行为 ---------- */
    /// Listen mode (inbound connections)
    #[arg(short, long)]
    pub listen: bool,

    /// Zero-I/O mode (scan for listening daemons)
    #[arg(short = 'z')]
    pub zero: bool,

    /// Keep inbound sockets open for multiple connects
    #[arg(short = 'k', long = "keep-open")]
    pub keep_open: bool,

    /// Do not resolve names (no DNS)
    #[arg(short = 'n', long = "numeric")]
    pub numeric: bool,

    /// Verbose
    #[arg(short, long)]
    pub verbose: bool,

    /* ---------- 超时/间隔 ---------- */
    /// Interval between lines sent (seconds)
    #[arg(short = 'i', long = "interval")]
    pub interval: Option<f64>,

    /// Timeout for connects and final net reads (seconds)
    #[arg(short = 'w', long = "timeout")]
    pub timeout: Option<f64>,

    /// Quit after N seconds of EOF on stdin
    #[arg(short = 'q', long = "quit-after")]
    pub quit_after: Option<u64>,

    /* ---------- 源地址/端口 ---------- */
    /// Source address to bind
    #[arg(short = 's', long = "source")]
    pub source: Option<String>,

    /// Source port (numeric)
    #[arg(short = 'p', value_parser = clap::value_parser!(u16))]
    pub source_port: Option<u16>,

    /* ---------- 代理 ---------- */
    /// Proxy protocol (e.g. "socks5", "http")
    #[arg(short = 'X', long = "proxy-protocol")]
    pub proxy_proto: Option<String>,

    /// Proxy address (host[:port])
    #[arg(short = 'x', long = "proxy")]
    pub proxy: Option<String>,

    /// Proxy username
    #[arg(short = 'P', long = "proxy-user")]
    pub proxy_username: Option<String>,

    /* ---------- 杂项 ---------- */
    /// TOS keyword
    #[arg(short = 'T', long = "tos")]
    pub tos: Option<String>,

    /// Routing table
    #[arg(short = 'V', long = "rtable")]
    pub rtable: Option<String>,

    /// Send buffer length (alias -I)
    #[arg(short = 'I', long = "send-length")]
    pub send_length: Option<usize>,

    /// Receive buffer length (alias -O)
    #[arg(short = 'O', long = "recv-length")]
    pub recv_length: Option<usize>,

    /* ---------- 新增：OpenBSD 其余单字母选项 ---------- */
    /// Allow broadcast
    #[arg(short = 'b', long = "broadcast")]
    pub broadcast: bool,

    /// Send CR+LF on line-feed
    #[arg(short = 'C', long = "crlf")]
    pub crlf: bool,

    /// Enable socket debugging
    #[arg(short = 'd', long = "debug")]
    pub debug: bool,

    /// Disable TCP delayed ack
    #[arg(short = 'D', long = "no-delay-ack")]
    pub no_delay_ack: bool,

    /// Print help (builtin)
    #[arg(short = 'h', long = "help", action = clap::ArgAction::Help)]
    pub help: bool,

    /// Randomize port numbers in range
    #[arg(short = 'r', long = "random")]
    pub random: bool,

    /// Enable TCP MD5 signature option (RFC 2385)
    #[arg(short = 'S', long = "md5sig")]
    pub md5sig: bool,

    /// Send RFC 854 DON’T/WON’T on stdin EOF
    #[arg(short = 't', long = "telnet")]
    pub telnet: bool,

    /* ---------- 位置参数 ---------- */
    /// Destination host (positional)
    #[arg(value_name = "destination")]
    pub destination: Option<String>,

    /// Destination port (positional)
    #[arg(value_name = "port", value_parser = clap::value_parser!(u16))]
    pub port: Option<u16>,
}