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
    pub cells: Vec<Vec<PuzzleCell>>,

    pub horiz_spans: Vec<Vec<usize>>,
    pub vert_spans: Vec<Vec<usize>>,
}

const SPAN_START_CHANCE: f64 = 0.3;
const SPAN_CONTINUE_CHANCE: f64 = 0.7;

impl Puzzle {
    pub fn new<R: ::rand::Rng>(width: usize, height: usize, rng: &mut R) -> Self {
        let mut cells = vec![vec![PuzzleCell::default(); width]; height];
        let mut horiz_spans = Vec::with_capacity(height);

        cells.iter_mut()
            .for_each(|ref mut row| {
                let mut pos = 0usize;
                let mut spans = vec![];

                while pos < row.len() {
                    if rng.gen::<f64>() <= SPAN_START_CHANCE {
                        pos += 1;
                        continue;
                    }

                    let start = pos;
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
                    spans.push(pos - start);
                    pos += 1;
                }

                horiz_spans.push(spans);
            });

        let mut vert_spans = Vec::with_capacity(width);
        for x in 0..width {
            let mut spans = vec![];

            let mut y = 0usize;
            while y < height {
                let start = y;
                while y < height && cells[y][x] == PuzzleCell::Filled {
                    y += 1;
                }

                if start != y {
                    spans.push(y - start);
                } else {
                    y += 1;
                }
            }
            vert_spans.push(spans);
        }


        Puzzle { 
            cells,
            horiz_spans,
            vert_spans,
        }
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
