#![feature(nll)]
#![feature(core_intrinsics)]
#![feature(duration_as_u128)]
#![feature(test)]

mod bank;
mod bitboard;
mod board;
mod mcts;
mod piece;
mod pieces;
mod player;
mod player_set;
mod shape;
mod placement;

use crate::bitboard::*;
use crate::board::*;
use crate::mcts::*;
use crate::player::*;
use crate::player_set::*;

use std::time::SystemTime;

use std::io::Write;

use rand::{rngs::SmallRng, FromEntropy, Rng};

use crate::shape::*;

fn main() {
    /*
    for piece in pieces::iter() {
        print!("Piece{{orientations:&[");
        for orientation in piece.orientations {
            let mut bits: u128 = 0;
            bits |= (orientation.bits[0] as u128);
            bits |= (orientation.bits[1] as u128) << 60;
            print!(
                "Shape {{bits:0x{:032X},attachments:&{:?},width:{},height:{}}},",
                bits, orientation.attachments, orientation.width, orientation.height
            );
        }
        print!("]}},")
    }*/

    let mut board = Board::new();
    let mut rng = SmallRng::from_entropy();
    let mut turn = Player::Blue;

    let mut out = PlayerSet::new();

    let mut file = std::fs::File::create("nodes.txt").unwrap();

    let mut node = Node::new(board, Player::Blue, PlayerSet::new());

    loop {
        node.display();
        println!();

        if node.is_terminal() {
            break;
        }

        let start = SystemTime::now();

        let mut count = 0;
        while SystemTime::now().duration_since(start).unwrap().as_secs() < 5 {
            for _ in 0..100 {
                node.step(&mut rng);
            }

            count += 100;
        }

        if node.is_terminal() {
            break;
        }

        println!("{:?} ({})", node.turn, count);

        let next = node.best_child(&mut rng);
        node = Node::new(next.board.clone(), next.turn, PlayerSet::new());
    }

    println!("{:?}", node.board.find_wins());
}
