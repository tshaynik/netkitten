use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub enum Options {
    Connect(ConnectOpts),
    Listen(ListenOpts),
    Scan(ScanOpts),
}

#[derive(StructOpt, Debug, Clone)]
pub struct ListenOpts {
    #[structopt(short, long)]
    execute: Option<String>,
    #[structopt(short, long)]
    keep_open: bool,
    host: String,
    port: u16,
}

#[derive(StructOpt, Debug, Clone)]
pub struct ConnectOpts {
    #[structopt(short, long)]
    execute: Option<String>,
    #[structopt(short, long)]
    keep_open: bool,
    host: String,
    port: u16,
}

#[derive(StructOpt, Debug, Clone)]
pub struct ScanOpts {
    host: String,
    port_range: u16,
}

pub fn listen(opt: &ListenOpts) -> io::Result<()> {
    if opt.keep_open {
        listen_concurrently(opt)
    } else {
        listen_once(opt)
    }
}

pub fn connect(opt: &ConnectOpts) -> io::Result<()> {
    let addr = format!("{}:{}", opt.host, opt.port);
    let stream = TcpStream::connect(addr)?;

    tcp_cat(stream)
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

fn tcp_cat(mut stream: TcpStream) -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut rstream = stream.try_clone()?;

    let whandle = spawn(move || std::io::copy(&mut stdin, &mut stream));
    let rhandle = spawn(move || std::io::copy(&mut rstream, &mut stdout));

    let _ = whandle.join().expect("writer fail");
    let _ = rhandle.join().expect("reader fail");

    Ok(())
}
