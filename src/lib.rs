mod connect;
mod listen;
mod scan;
mod transfer;

pub use connect::connect;
pub use listen::listen;
pub use scan::scan;

use connect::ConnectOpts;
use listen::ListenOpts;
use scan::ScanOpts;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub enum Options {
    Connect(ConnectOpts),
    Listen(ListenOpts),
    Scan(ScanOpts),
}
