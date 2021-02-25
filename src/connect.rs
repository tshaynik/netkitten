use crate::transfer::tcp_cat;
use anyhow::{Context, Result};
use std::net::TcpStream;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct ConnectOpts {
    #[structopt(short, long)]
    execute: Option<String>,
    #[structopt(short, long)]
    keep_open: bool,
    host: String,
    port: u16,
}

pub fn connect(opt: &ConnectOpts) -> Result<()> {
    let addr = format!("{}:{}", opt.host, opt.port);
    let stream = TcpStream::connect(addr)?;

    tcp_cat(stream).with_context(|| "failed to stream to/from socket")
}
