use crate::shape::Shape;

#[derive(Debug)]
pub struct Piece {
    pub orientations: &'static [Shape],
}

pub type PieceId = u8;
