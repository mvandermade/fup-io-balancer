use ::clap::Parser;
use ::std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(
    about = "Run the postzegel load balancer",
)]
pub struct CliArgs {
    /// Run in verbose mode
    #[arg(short = 'v', long)]
    pub verbose: bool,
    /// Run in quiet mode
    #[arg(short = 'q', long, conflicts_with = "verbose")]
    pub quiet: bool,
    /// The ip and port to listen on
    #[arg(short = 'b', long, default_value = "0.0.0.0:7331")]
    pub addr: SocketAddr,
    /// Time to wait in microseconds when there are no workers
    #[arg(long, default_value = "500000")]
    pub no_worker_delay_us: u64,

}

#[test]
fn test_cli_args() {
    CliArgs::try_parse_from(["cmd", "-v"]).unwrap();
    CliArgs::try_parse_from(["cmd", "-q"]).unwrap();
    CliArgs::try_parse_from(["cmd", "-b", "127.0.0.1:8080"]).unwrap();
    CliArgs::try_parse_from(["cmd", "-b", "127.0.0.1:8080", "--no-worker-delay-us=1"]).unwrap();
}
