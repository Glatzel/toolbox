#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Thaat {
    Bilaval,
    Khamaj,
    Kafi,
    Asavari,
    Bhairavi,
    Kalyan,
    Marva,
    Poorvi,
    Todi,
    Bhairav,
}

impl Thaat {
    pub const fn intervals(self) -> &'static [u8; 7] {
        match self {
            Self::Bilaval => &[0, 2, 4, 5, 7, 9, 11],
            Self::Khamaj => &[0, 2, 4, 5, 7, 9, 10],
            Self::Kafi => &[0, 2, 3, 5, 7, 9, 10],
            Self::Asavari => &[0, 2, 3, 5, 7, 8, 10],
            Self::Bhairavi => &[0, 1, 3, 5, 7, 8, 10],
            Self::Kalyan => &[0, 2, 4, 6, 7, 9, 11],
            Self::Marva => &[0, 1, 4, 6, 7, 9, 11],
            Self::Poorvi => &[0, 1, 4, 6, 7, 8, 11],
            Self::Todi => &[0, 1, 3, 6, 7, 8, 11],
            Self::Bhairav => &[0, 1, 4, 5, 7, 8, 11],
        }
    }
}