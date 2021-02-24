use std::io;
use std::net::TcpStream;
use std::thread::spawn;

pub fn tcp_cat(mut stream: TcpStream) -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut rstream = stream.try_clone()?;

    let whandle = spawn(move || std::io::copy(&mut stdin, &mut stream));
    let rhandle = spawn(move || std::io::copy(&mut rstream, &mut stdout));

    let _ = whandle.join().expect("writer fail");
    let _ = rhandle.join().expect("reader fail");

    Ok(())
}
