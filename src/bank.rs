use crate::piece::*;
use crate::pieces;
use std::intrinsics;
use std::iter;

#[derive(Clone, Debug)]
pub struct Bank {
    pub pieces: u32,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            pieces: 0x00000000001FFFFF,
        }
    }

    pub fn take(&mut self, piece: usize) {
        self.pieces &= !(1 << piece);
    }

    pub fn take_iter(&self) -> TakeIter {
        TakeIter {
            pieces: self.pieces,
            remaining: self.pieces,
        }
    }
}

pub struct TakeIter {
    pieces: u32,
    remaining: u32,
}

impl iter::Iterator for TakeIter {
    type Item = (&'static Piece, usize);

    fn next(&mut self) -> Option<(&'static Piece, usize)> {
        if self.remaining == 0 {
            None
        } else {
            let piece = unsafe { intrinsics::cttz(self.remaining) as usize };
            self.remaining &= !(1 << piece);
            Some((pieces::by_id(piece as PieceId), piece))
        }
    }
}
