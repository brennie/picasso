extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod puzzle;

use structopt::StructOpt;

use puzzle::Puzzle;

#[derive(Debug, StructOpt)]
struct Options {
    /// The desired width of the generated puzzle.
    #[structopt(name = "WIDTH")]
    width: usize,

    // The desired height of the generated puzzle.
    #[structopt(name = "HEIGHT")]
    height: usize,
}

fn main() {
    let options = Options::from_args();
    let puzzle = Puzzle::new(options.width, options.height, &mut rand::thread_rng());
    println!("{}\n", puzzle);
}
