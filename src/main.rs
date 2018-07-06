extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod puzzle;

use std::cmp;

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

fn digit_len(n: usize) -> usize {
    (n as f32).log10().floor() as usize + 1
}

fn main() {
    let options = Options::from_args();
    let puzzle = Puzzle::new(options.width, options.height, &mut rand::thread_rng());

    let max_vert_span_count = puzzle.vert_spans.iter().map(Vec::len).fold(0, cmp::max);
    let max_horiz_span_count = puzzle.horiz_spans.iter().map(|xs| {
        xs.iter().map(|span| digit_len(*span) + 1).sum::<usize>()
        // xs.iter().map(|span| span.to_string().len() + 1).sum::<usize>()
    }).fold(0, cmp::max);

    for s in 0..max_vert_span_count {
        for _ in 0..max_horiz_span_count {
            print!(" ");
        }

        for x in 0..puzzle.cells[0].len() {
            let s = puzzle.vert_spans[x].get(s)
                .map(ToString::to_string)
                .unwrap_or_else(|| String::from(" "));
            
            print!("{} ", s);
        }

        println!();
    }

    for y in 0..puzzle.cells.len() {
        let mut pos = 0;

        for span in &puzzle.horiz_spans[y] {
            print!("{} ", span);
            pos += digit_len(*span) + 1;
        }

        while pos < max_horiz_span_count {
            print!(" ");
            pos += 1;
        }

        for cell in &puzzle.cells[y] {
            print!("{} ", cell);
        }

        println!();
    }

    print!("{}", max_horiz_span_count);
}
