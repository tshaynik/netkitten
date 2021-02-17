use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct Options {
    host: String,
    port: u16,
    #[structopt(short)]
    pub listen: bool,
    #[structopt(short)]
    upload: bool,
    #[structopt(short)]
    command: bool,
    #[structopt(short)]
    execute: Option<String>,
    #[structopt(short = "d")]
    upload_destination: Option<String>,
}

pub fn listen(opt: &Options) {
    let listener = TcpListener::bind(format!("{}:{}", opt.host, opt.port)).unwrap();

    loop {
        let (socket, _) = listener.accept().unwrap();

        std::thread::spawn(move || connect(socket).unwrap());
    }
}

pub fn client(opt: &Options) -> io::Result<()> {
    let addr = format!("{}:{}", opt.host, opt.port);
    let stream = TcpStream::connect(addr)?;

    connect(stream)
}

pub fn connect(mut stream: TcpStream) -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut rstream = stream.try_clone()?;

    let whandle = spawn(move || std::io::copy(&mut stdin, &mut stream));
    let rhandle = spawn(move || std::io::copy(&mut rstream, &mut stdout));

    let _ = whandle.join().expect("writer fail");
    let _ = rhandle.join().expect("reader fail");

    Ok(())
}
