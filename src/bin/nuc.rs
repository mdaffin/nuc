extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate nuc;

use nuc::Number;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "nuc", about = "Converts numbers form one for to another")]
struct Opt {
    /// Number to convert
    #[structopt(help = "Input numbers")]
    input: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    for arg in opt.input {
        let num: Number = match arg.parse() {
            Ok(i) => i,
            Err(e) => {
                println!("Failed to parse input '{}': {}", arg, e);
                std::process::exit(1);
            }
        };
        print!("{:>8} ", num);
        print!("{:>8} ", num.fmt_signed());
        print!("{:>#8x} ", num);
        print!("{:>#16b}", num);
        println!();
    }
}
