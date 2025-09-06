use crate::figure::{Figure, FigureColor};

pub struct Board {
    cells: Vec<Vec<Option<(Figure, FigureColor)>>>,
}

impl Board {
    pub fn new() -> Self {
        let mut cells = vec![vec![None; 8]; 8];

        for x in 0..8 {
            cells[1][x] = Some((Figure::Pawn, FigureColor::Black));
            cells[6][x] = Some((Figure::Pawn, FigureColor::White));
        }

        let back_row = [
            Figure::Rook,
            Figure::Knight,
            Figure::Bishop,
            Figure::Queen,
            Figure::King,
            Figure::Bishop,
            Figure::Knight,
            Figure::Rook,
        ];

        for x in 0..8 {
            cells[0][x] = Some((back_row[x].clone(), FigureColor::Black));
            cells[7][x] = Some((back_row[x].clone(), FigureColor::White));
        }

        Board { cells }
    }

    pub fn display(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Some((figure, color)) => {
                        let symbol = figure.symbol();
                        let display_char = match color {
                            FigureColor::White => symbol,
                            FigureColor::Black => symbol.to_ascii_lowercase(),
                        };
                        print!(" {} ", display_char);
                    }
                    None => print!(" . "),
                }
            }
            println!();
        }
    }
}
