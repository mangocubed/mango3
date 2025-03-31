#[derive(Clone, Eq, PartialEq)]
pub enum ActionFormStatus {
    Error,
    Pending,
    Success,
    Done,
}

impl ActionFormStatus {
    pub fn is_done(&self) -> bool {
        *self == Self::Done
    }

    pub fn is_error(&self) -> bool {
        *self == Self::Error
    }

    pub fn is_pending(&self) -> bool {
        *self == Self::Pending
    }

    pub fn is_success(&self) -> bool {
        *self == Self::Success
    }
}

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
