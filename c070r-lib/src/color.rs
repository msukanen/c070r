use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    _BLANK,
    /// Red
    R,
    /// Green
    G,
    /// Blue
    B,
    /// Yellow
    Y,
    /// Black
    X,
    /// White
    O,
}

impl Color {
    pub fn blank() -> Self { Self::_BLANK }
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..6) {
            ..=0 => Self::B,
            1 => Self::G,
            2 => Self::O,
            3 => Self::R,
            4 => Self::X,
            5 => Self::Y,
            n => todo!("Value {n} out of color index range!")
        }
    }
}
