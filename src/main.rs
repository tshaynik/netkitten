use netkitten::{client, listen, Options};
use structopt::StructOpt;

fn main() {
    let opt = Options::from_args();

    if opt.listen {
        listen(&opt);
    } else {
        client(&opt);
    }
}
