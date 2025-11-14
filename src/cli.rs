use argh::FromArgs;
use std::net::IpAddr;

/// Ground-up implementation of a nano HTTP server from TCP sockets.
#[derive(FromArgs, Debug)]
#[argh(help_triggers("-h", "--help", "help"))]
pub struct Cli {
    /// IP address to bind the server to
    #[argh(option, default = "IpAddr::from([127, 0, 0, 1])", short = 'a')]
    pub address: IpAddr,
    /// port to bind the server to
    #[argh(option, default = "8080", short = 'p')]
    pub port: u16,
}
