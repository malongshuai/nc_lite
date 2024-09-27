use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "tcp_tool", about = "A TCP client-server tool.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run as a TCP server
    #[clap(visible_alias = "s")]
    Server {
        /// Address to bind the server, format: "127.0.0.1:44149"
        #[arg(short = 's', long = "addr")]
        addr: String,

        /// Keep the server running after reading one data
        #[arg(short = 'k')]
        keep: bool,
    },
    /// Run as a TCP client
    #[clap(visible_alias = "c")]
    Client {
        /// Server address to send data
        #[arg(short = 's', long = "addr")]
        addr: String,
        /// Data to send to the server
        data: Vec<String>,
    },
}
