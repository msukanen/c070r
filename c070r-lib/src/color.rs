use rand::Rng;

/// Some "arbitrary" colors.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// Blank &ndash; &ldquo;not-a-color&rdquo;.
    // TODO: purely an alpha channel?
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
    /// Generate a "blank" [Color].
    pub fn blank() -> Self { Self::_BLANK }
    /// Generate a random [Color].
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

    pub fn as_rgba(&self) -> (u32, u32, u32, u32) {
        match self {
            Self::B => (0, 0, 255, 255),
            Self::G => (0, 255, 0, 255),
            Self::O => (255, 255, 255, 255),
            Self::R => (255, 0, 0, 255),
            Self::X => (0, 0, 0, 255),
            Self::Y => (255, 255, 0, 255),

            Self::_BLANK => (0, 0, 0, 0)
        }
    }
}

impl std::fmt::Display for Color {
    /// ASCII-print the `Color`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::B => "B",
            Self::G => "G",
            Self::O => "#",
            Self::R => "R",
            Self::Y => "Y",
            Self::X => "-",
            Self::_BLANK => " ",
        })
    }
}
