#[derive(Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn is_horizontal(&self) -> bool {
        *self == Self::Horizontal
    }

    pub fn is_vertical(&self) -> bool {
        *self == Self::Vertical
    }
}
