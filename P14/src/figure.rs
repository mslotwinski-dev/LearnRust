#[derive(Clone)]
pub enum Figure {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone)]
pub enum FigureColor {
    White,
    Black,
}

impl Figure {
    pub fn symbol(&self) -> char {
        match self {
            Figure::Pawn => 'P',
            Figure::Rook => 'R',
            Figure::Knight => 'N',
            Figure::Bishop => 'B',
            Figure::Queen => 'Q',
            Figure::King => 'K',
        }
    }

    fn value(&self) -> u8 {
        match self {
            Figure::Pawn => 1,
            Figure::Rook => 5,
            Figure::Knight => 3,
            Figure::Bishop => 3,
            Figure::Queen => 9,
            Figure::King => 0, // King is invaluable
        }
    }
}
