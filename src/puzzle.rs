use rand::distributions::StandardNormal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PuzzleCell {
    Filled,
    Space,
}

impl ::std::fmt::Display for PuzzleCell {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::PuzzleCell::*;

        let c = match self {
            Filled => '■',
            Space => '╳',
        };

        write!(f, "{}", c)
    }
}

impl ::std::default::Default for PuzzleCell {
    fn default() -> PuzzleCell {
        PuzzleCell::Space
    }
}

/// A nonogram puzzle.
#[derive(Debug)]
pub struct Puzzle {
    cells: Vec<Vec<PuzzleCell>>,
}

const SPAN_START_CHANCE: f64 = 0.3;
const SPAN_CONTINUE_CHANCE: f64 = 0.7;

impl Puzzle {
    pub fn new<R: ::rand::Rng>(width: usize, height: usize, rng: &mut R) -> Self {
        let mut cells = vec![vec![PuzzleCell::default(); width]; height];

        cells.iter_mut()
            .for_each(|ref mut row| {
                let mut pos = 0usize;

                while pos < row.len() {
                    if rng.gen::<f64>() <= SPAN_START_CHANCE {
                        pos += 1;
                        continue;
                    }

                    row[pos] = PuzzleCell::Filled;
                        pos += 1;

                    while pos < row.len() {
                        if rng.sample(StandardNormal) <= SPAN_CONTINUE_CHANCE {
                            row[pos] = PuzzleCell::Filled;
                            pos += 1;
                        } else {
                            break
                        }
                    }
                    pos += 1;
                }
            });

        Puzzle { cells }
    }
}

impl ::std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| write!(f, "{}", cell))
                    .collect::<Result<_, _>>()?;
                write!(f, "\n")
            })
            .collect()
    }
}
