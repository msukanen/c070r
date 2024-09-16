use crate::color::Color;

pub struct BoardMatrix {
    /// The matrix. Fixed size, for now - shall see about dynamic sizing eventually™.
    matrix: [[Color; BoardMatrix::WIDTH]; BoardMatrix::HEIGHT],
}

impl BoardMatrix {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;

    /// Generate a new, blank board.
    pub fn blank() -> Self {
        Self { matrix: [
            [Color::blank(), Color::blank(), Color::blank(), Color::blank(), Color::blank()],
            [Color::blank(), Color::blank(), Color::blank(), Color::blank(), Color::blank()],
            [Color::blank(), Color::blank(), Color::blank(), Color::blank(), Color::blank()],
            [Color::blank(), Color::blank(), Color::blank(), Color::blank(), Color::blank()],
            [Color::blank(), Color::blank(), Color::blank(), Color::blank(), Color::blank()]
        ] }
    }

    /// Generate a new, randomly colorized board.
    pub fn new() -> Self {
        Self { matrix: [
            [Color::random(), Color::random(), Color::random(), Color::random(), Color::random()],
            [Color::random(), Color::random(), Color::random(), Color::random(), Color::random()],
            [Color::random(), Color::random(), Color::random(), Color::random(), Color::random()],
            [Color::random(), Color::random(), Color::random(), Color::random(), Color::random()],
            [Color::random(), Color::random(), Color::random(), Color::random(), Color::random()]
        ] }
    }

    /**
     Validate given `x|y`.
        
     **Panic**
     if out of bounds.
     */
    fn validate_xy(x: usize, y: usize) {
        if BoardMatrix::is_out_of_bounds(x, y) {
            panic!("x|y of {x}|{y} outside legal range!")
        }
    }

    /// Check whether given `x|y` are within board bounds.
    fn is_out_of_bounds(x: usize, y: usize) -> bool {
        x >= BoardMatrix::WIDTH || y >= BoardMatrix::HEIGHT
    }

    /// Set `x|y` contents to `c`.
    pub fn set(&mut self, x: usize, y: usize, c: &Color) {
        BoardMatrix::validate_xy(x, y);
        self.matrix[x][y] = c.clone();
    }

    /**
     Erase `x|y` and all connected pieces that match `c`.
     
     **Returns** count of erased pieces.
     */
    // There isn't much of "heuristics" to speak of, just simple, unoptimized and crude surroundings check.
    pub fn erase_connected(&mut self, x: usize, y: usize, c: &Color) -> usize {
        let mut erased = 0;
        if !BoardMatrix::is_out_of_bounds(x, y) && self.matrix[x][y].eq(&c) {
            erased += 1;
            self.matrix[x][y] = Color::_BLANK;
            if x > 0 {
                erased += self.erase_connected(x - 1, y, &c)
            }
            if y > 0 {
                erased += self.erase_connected(x, y - 1, &c)
            }

            erased += self.erase_connected(x + 1, y, &c);// note: we allow `x` to go out of bounds, it's safe.
            erased += self.erase_connected(x, y + 1, &c);// note: we allow `y` to go out of bounds, it's safe.
        }
        erased
    }

    /// Get what's at `x|y`.
    /// 
    /// **Panic**
    /// if out of bounds.
    pub fn at(&self, x: usize, y: usize) -> Color {
        BoardMatrix::validate_xy(x, y);
        self.matrix[x][y].clone()
    }
}

#[cfg(test)]
mod board_matrix_tests {
    use crate::color::Color;

    use super::BoardMatrix;

    #[test]
    fn init_works() {
        let bm = BoardMatrix::blank();
        for xy in &bm.matrix {
            for c in xy {
                assert_eq!(c, &Color::_BLANK);
            }
        }
    }

    #[test]
    #[should_panic]
    fn set_out_of_range_panics() {
        let mut bm = BoardMatrix::blank();
        bm.set(9, 9, &Color::B);
    }

    #[test]
    fn set_works() {
        let mut bm = BoardMatrix::new();
        match bm.at(2, 2) {
            Color::R => {
                bm.set(2, 2, &Color::B);
                assert_eq!(bm.at(2, 2), Color::B);
            },
            _ => {
                bm.set(2, 2, &Color::R);
                assert_eq!(bm.matrix[2][2], Color::R);
            }
        }
    }

    #[test]
    fn erase_connected_works() {
        let mut bm = BoardMatrix::blank();
        
        let c = Color::G;
        // let's form a 5-high 3-wide "cross"; total 7 `c` colors.
        for y in 0..BoardMatrix::HEIGHT {
            bm.set(2, y, &c);
        }
        bm.set(1, 2, &c);
        bm.set(3, 2, &c);

        let c = bm.at(2, 2);
        let erased = bm.erase_connected(2, 2, &c);
        // vertical mid-line is all _BLANK now, as it should?
        for y in 0..BoardMatrix::HEIGHT {
            assert_eq!(Color::_BLANK, bm.at(2, y))
        }
        assert_eq!(7, erased);
    }

    #[test]
    fn erase_outofbounds_silently_ignored() {
        let mut bm = BoardMatrix::blank();
        let erased = bm.erase_connected(BoardMatrix::WIDTH + 10, BoardMatrix::HEIGHT + 10, &Color::B);
        assert_eq!(5, erased);
    }
}

impl std::fmt::Display for BoardMatrix {
    /// ASCII-print the matrix.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn line() -> String {
            let mut line = vec!["+"];
            for _ in 0..BoardMatrix::WIDTH {
                line.push("-")
            }
            line.push("+\n");
            line.join("").as_str().to_string()
        }

        write!(f, "{}", {
            let mut output = vec![line()];
            for xy in &self.matrix {
                output.push("|".to_string());
                for y in xy {
                    output.push(y.to_string())
                }
                output.push("|\n".to_string())
            }
            output.push(line());
            output.join("").as_str()
        })
    }
}
