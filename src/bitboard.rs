use std::intrinsics;

use crate::pieces;
use crate::placement::Placement;
use crate::player::Player;
use crate::shape::Shape;

const WEST_MASK: u128 = 0x007F_FFF7_FFFF_7FFF_F7FF_FF7F_FFF7_FFFF;
const EAST_MASK: u128 = 0xFEFF_FFEF_FFFE_FFFF_EFFF_FEFF_FFEF_FFFE;
const BOTTOM_MASK: u128 = 0x00FF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
const ROW_MASK: u128 = 0x0000_0000_0000_0000_0000_0000_000F_FFFF;
const FIRST_MASK: u128 = 0x0000_0000_0000_0000_0000_0000_000F_FFFE;
const REST_MASK: u128 = 0x0000_0000_0000_0000_0000_0000_0007_FFFF;
const HALF_MASK: u128 = 0x0000_0000_0000_0000_0000_00FF_FFFF_FFFF;

const TOP_CORNERS: u128 = 0x0000_0000_0000_0000_0080_0010_0000;
const BOTTOM_CORNERS: u128 = 0x0000_0000_0000_0000_0000_0000_0008_0001;

#[derive(Clone, PartialEq, Debug)]
pub struct BitBoard {
    pub blocks: [u128; 4],
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard {
            blocks: [0, 0, 0, 0],
        }
    }

    pub fn illegal(&self, player: Player, boards: &[BitBoard; 4]) -> Self {
        let mut enemy = BitBoard::new();
        let mut turn = player;
        for turn in Player::iter_from(player.next()) {
            for i in 0..4 {
                enemy.blocks[i] |= boards[turn as usize].blocks[i];
            }
        }

        let enemy = enemy;

        let mut board = self.clone();

        let mut block = self.blocks[0];
        let mut flood;
        let mut prop = 0;

        // TODO: Manually unroll
        for i in 0..3 {
            flood = prop;
            flood |= (block >> 1) & WEST_MASK;
            flood |= (block << 1) & EAST_MASK;
            flood |= block >> 20;
            flood |= block << 20;
            prop = block >> 100;
            block = self.blocks[i + 1];
            flood |= (block & ROW_MASK) << 100;

            board.blocks[i] |= (flood & BOTTOM_MASK) | enemy.blocks[i];
        }

        flood = prop;
        flood |= (block >> 1) & WEST_MASK;
        flood |= (block << 1) & EAST_MASK;
        flood |= block << 20;
        flood |= block >> 20;

        board.blocks[3] |= (flood & HALF_MASK) | enemy.blocks[3];
        board
    }

    pub fn corners(&self, illegal: &BitBoard) -> Self {
        let mut board = BitBoard::new();

        let mut block = self.blocks[0];
        let mut flood;
        let mut prop = 0;

        for i in 0..3 {
            flood = prop;
            flood |= (block >> 21) & WEST_MASK;
            flood |= (block << 19) & WEST_MASK;
            flood |= (block << 21) & EAST_MASK;
            flood |= (block >> 19) & EAST_MASK;
            prop = ((block >> 99) & FIRST_MASK) | ((block >> 101) & REST_MASK);
            block = self.blocks[i + 1];
            flood |= (((block >> 1) & WEST_MASK) | ((block << 1) & EAST_MASK)) << 100;
            board.blocks[i] = flood & BOTTOM_MASK & !illegal.blocks[i];
        }

        flood = prop;
        flood |= (block >> 21) & WEST_MASK;
        flood |= (block << 19) & WEST_MASK;
        flood |= (block << 21) & EAST_MASK;
        flood |= (block >> 19) & EAST_MASK;
        board.blocks[3] = flood & HALF_MASK & !illegal.blocks[3];

        board.blocks[0] |= BOTTOM_CORNERS & !illegal.blocks[0];
        board.blocks[3] |= TOP_CORNERS & !illegal.blocks[3];

        board
    }

/*
    pub fn starting_corners(&self, player: Player, illegal: &BitBoard) -> BitBoard {
        let mut corners = BitBoard::new();
        corners.blocks[0] = BOTTOM_CORNERS;
        corners.blocks[3] = TOP_CORNERS;

        let mut turn = player;
        corners.blocks[0] &= !illegal.blocks[0];
        corners.blocks[3] &= !illegal.blocks[3];

        corners
    }*/

    pub fn count_tiles(&self) -> usize {
        self.blocks
            .iter()
            .map(|&block| unsafe { intrinsics::ctpop(block) as usize })
            .sum()
    }

    pub fn place(&mut self, placement: &Placement) {
        let shape = &pieces::PIECES[placement.piece()].orientations[placement.orientation()];
        let shift = placement.shift();
        let shifted = shape.bits << placement.shift();
        let block = placement.block();
        if block == 3 {
            self.blocks[block] |= shifted & HALF_MASK;
        } else {
            self.blocks[block] |= shifted & BOTTOM_MASK;
            let shifted = shape.bits >> (120 - shift);

            // TODO: see if this is actually necessary. I doubt it is.
            let mask = if block == 2 { HALF_MASK } else { BOTTOM_MASK };
            self.blocks[block + 1] |= shifted & mask;
        }
    }

    pub fn make_placement(
        &self,
        piece_id: usize,
        shape: &Shape,
        orientation_id: usize,
        attachment: u8,
        at: usize,
        illegal: &BitBoard,
    ) -> Option<Placement> {
        if at < (attachment as usize) {
            return None;
        }

        let index = at - (attachment as usize);

        if (index % 20) + (shape.width as usize) >= 20 {
            return None;
        }

        if index + (shape.height as usize) * 20 >= 400 {
            return None;
        }

        let block = index / 120;
        let shift = index % 120;

        let placement = Placement::new(block, shift, piece_id, orientation_id);

        let shifted = shape.bits << shift;
        if (shifted & illegal.blocks[block]) != 0 {
            return None;
        }

        if block == 3 {
            if (shifted & HALF_MASK) != shifted {
                None
            } else {
                Some(placement)
            }
        } else {
            let shifted = shape.bits >> (120 - shift);
            if (shifted & illegal.blocks[block + 1]) != 0 {
                None
            } else {
                Some(placement)
            }
        }
    }

/*
    pub fn place_shape(
        &self,
        shape: &Shape,
        attachment: &u8,
        at: usize,
        illegal: &BitBoard,
    ) -> Option<Self> {
        if at < (*attachment as usize) {
            return None;
        }

        let index = at - (*attachment as usize);

        if (index % 20) + (shape.width as usize) >= 20 {
            return None;
        }

        if index + (shape.height as usize) * 20 >= 400 {
            return None;
        }

        // TODO: We're copying more than we need to, earlier than we need to. Make this more
        // efficient.
        let mut copy = self.clone();

        let block = index / 120;
        let shift = index % 120;

        let shifted = shape.bits << shift;
        if (shifted & illegal.blocks[block]) != 0 {
            return None;
        }

        copy.blocks[block] |= shifted & BOTTOM_MASK;

        if block == 3 {
            if (shifted & HALF_MASK) != shifted {
                None
            } else {
                Some(copy)
            }
        } else {
            let shifted = shape.bits >> (120 - shift);
            if (shifted & illegal.blocks[block + 1]) != 0 {
                None
            } else {
                copy.blocks[block + 1] |= shifted & BOTTOM_MASK;
                Some(copy)
            }
        }
    }*/

    pub fn display(&self) {
        for block in self.blocks.iter().rev() {
            for y in (if *block == self.blocks[3] { 4 } else { 0 })..6 {
                for x in 0..20 {
                    let s = (5 - y) * 20 + x;
                    print!("{}", (block >> s) & (1 as u128));
                }

                println!();
            }
        }
    }

    pub fn is_occupied(&self, at: usize) -> bool {
        ((self.blocks[at / 120] >> (at % 120)) & 1) == 1
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(|&b| b == 0)
    }

    pub fn iter(&self) -> BitIterator<'_> {
        BitIterator {
            block: 0,
            blocks: &self.blocks,
            bits: self.blocks[0],
        }
    }
}

pub struct BitIterator<'a> {
    block: u8,
    blocks: &'a [u128; 4],
    bits: u128,
}

impl<'a> BitIterator<'a> {
    fn new(board: &'a BitBoard) -> Self {
        BitIterator {
            block: 0,
            blocks: &board.blocks,
            bits: board.blocks[0],
        }
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            let index = unsafe { intrinsics::cttz(self.bits) };

            if index == 128 {
                if self.block == 3 {
                    return None;
                } else {
                    self.block += 1;
                    self.bits = self.blocks[self.block as usize];
                }
            } else {
                self.bits &= !((1 as u128) << index);
                return Some((self.block as usize) * 120 + (index as usize));
            }
        }
    }
}
