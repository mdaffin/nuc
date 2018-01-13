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
        let num: Number = arg.parse().expect("failed to parse input");
        print!("{}\t", num);
        print!("{:#x}\t", num);
        print!("{:#b}", num);
        println!();
    }
}
