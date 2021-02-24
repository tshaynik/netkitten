use super::transfer::tcp_cat;

use std::io;
use std::net::TcpListener;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct ListenOpts {
    #[structopt(short, long)]
    execute: Option<String>,
    #[structopt(short, long)]
    keep_open: bool,
    host: String,
    port: u16,
}

pub fn listen(opt: &ListenOpts) -> io::Result<()> {
    if opt.keep_open {
        listen_concurrently(opt)
    } else {
        listen_once(opt)
    }
}

fn listen_once(opt: &ListenOpts) -> io::Result<()> {
    let addr = format!("{}:{}", opt.host, opt.port);
    let listener = TcpListener::bind(addr)?;
    let (socket, _) = listener.accept()?;
    tcp_cat(socket)
}

fn listen_concurrently(opt: &ListenOpts) -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", opt.host, opt.port))?;

    loop {
        let (socket, _) = listener.accept().unwrap();

        std::thread::spawn(move || tcp_cat(socket).unwrap());
    }
}
