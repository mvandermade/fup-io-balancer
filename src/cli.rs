use std::net::SocketAddr;
use ::clap::Parser;

#[derive(Parser, Debug)]
#[command(
    about = "Run the postzegel load balancer",
)]
pub struct CliArgs {
    /// Run in verbose mode
    #[arg(short = 'v')]
    pub verbose: bool,
    /// Run in quiet mode
    #[arg(short = 'q', conflicts_with = "verbose")]
    pub quiet: bool,
    /// The ip and port to listen on
    #[arg(short = 'b', default_value = "127.0.0.1:7331")]
    pub addr: SocketAddr,
}

#[test]
fn test_cli_args() {
    CliArgs::try_parse_from(&["cmd", "-v"]).unwrap();
    CliArgs::try_parse_from(&["cmd", "-q"]).unwrap();
}
