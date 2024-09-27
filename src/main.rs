//! 简单的nc命令，可做服务端监听并打印接受的内容，也可以做客户端向服务端发送数据

use clap::Parser;
use opts::{Cli, Commands};
use std::{
    io::{BufRead, BufReader, Write as _},
    net::{TcpListener, TcpStream},
};

pub mod opts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Server { addr, keep } => {
            run_server(addr, *keep)?;
        }
        Commands::Client { addr, data } => {
            let mut str = data.join(" ");
            str.push('\n');
            run_client(addr, str)?;
        }
    }

    Ok(())
}

fn run_server(addr: &str, keep: bool) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr)?;

    'outer: loop {
        let (socket, _) = listener.accept()?;
        let mut socket = BufReader::new(socket);
        let mut buf = String::new();
        loop {
            match socket.read_line(&mut buf) {
                Err(_e) => break,
                Ok(0) => break, // Connection closed
                Ok(_n) => {
                    print!("{buf}");
                    buf.clear();
                }
            }
            if !keep {
                break 'outer; // Client disconnected
            }
        }
    }
    Ok(())
}

fn run_client(addr: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr)?;

    stream.write_all(data.as_bytes())?;

    Ok(())
}
