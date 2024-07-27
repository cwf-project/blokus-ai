use crate::bank::*;
use crate::bitboard::*;
use crate::pieces;
use crate::placement::*;
use crate::player;
use crate::player::Player;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Board {
    pub placed: [BitBoard; 4],
    banks: [Bank; 4],
}

impl Board {
    pub fn new() -> Self {
        Board {
            placed: [
                BitBoard::new(),
                BitBoard::new(),
                BitBoard::new(),
                BitBoard::new(),
            ],
            banks: [Bank::new(), Bank::new(), Bank::new(), Bank::new()],
        }
    }

    pub fn find_moves(&self, player: Player) -> Vec<Placement> {
        let board = &self.placed[player as usize];
        let illegal = board.illegal(player, &self.placed);
        let corners = board.corners(&illegal);

        let mut moves = Vec::new();
        let bank = &self.banks[player as usize];

        for corner in corners.iter() {
            for (piece, piece_id) in bank.take_iter() {
                for (orientation_id, orientation) in piece.orientations.iter().enumerate() {
                    for attachment in orientation.attachments.iter() {
                        if let Some(placement) = board.make_placement(
                            piece_id,
                            orientation,
                            orientation_id,
                            *attachment,
                            corner,
                            &illegal,
                        ) {
                            moves.push(placement);
                        }
                    }
                }
            }
        }

        moves
    }

    pub fn perform_placement(&mut self, placement: &Placement, player: Player) {
        self.placed[player as usize].place(placement);
        self.banks[player as usize].take(placement.piece());
    }

    pub fn play_randomly<R: Rng>(&mut self, player: Player, rng: &mut R) -> bool {
        let moves = self.find_moves(player);
        if moves.is_empty() {
            false
        } else {
            let placement = rng.choose(&moves).unwrap();
            self.perform_placement(placement, player);

            true
        }
    }

    pub fn find_wins(&self) -> [f64; 4] {
        let mut winning_score = 0;
        let mut occurances = 0;

        for turn in Player::iter() {
            let score = self.score(turn);
            if score > winning_score {
                winning_score = score;
                occurances = 1;
            } else if score == winning_score {
                occurances += 1;
            }
        }

        let mut wins: [f64; 4] = [0.0, 0.0, 0.0, 0.0];

        for turn in Player::iter() {
            if self.score(turn) == winning_score {
                wins[turn as usize] += 1.0 / (occurances as f64);
            }
        }

        wins
    }

    pub fn display(&self) {
        for y in 0..20 {
            for x in 0..20 {
                let index = (19 - y) * 20 + x;
                if self.placed[0].is_occupied(index) {
                    print!("\x1b[106m");
                } else if self.placed[1].is_occupied(index) {
                    print!("\x1b[103m");
                } else if self.placed[2].is_occupied(index) {
                    print!("\x1b[101m");
                } else if self.placed[3].is_occupied(index) {
                    print!("\x1b[102m");
                } else {
                    print!("\x1b[100m");
                }

                print!(" ");
                //print!("\u{2001}");
            }

            print!("\x1b[49m");
            println!();
        }
    }

    pub fn score(&self, player: Player) -> usize {
        self.placed[player as usize].count_tiles()
    }
}
