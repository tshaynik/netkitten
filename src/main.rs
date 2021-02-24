use anyhow::Result;
use netkitten::{connect, listen, Options};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Options::from_args();

    match opt {
        Options::Listen(listen_opts) => listen(&listen_opts),
        Options::Connect(connect_opts) => connect(&connect_opts),
        Options::Scan(_scan_opts) => {
            unimplemented!();
        }
    }?;

    Ok(())
}
