use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct ScanOpts {
    host: String,
    port_range: u16,
}
