use crate::board::*;
use crate::player;
use crate::player::*;
use crate::player_set::*;

use rand::Rng;

const EXPLORATION: f64 = 0.4;
const EPSILON: f64 = 0.0001;

#[derive(Clone, Debug)]
pub struct Node {
    pub board: Board,
    pub turn: Player,
    visits: u32,
    wins: [f64; 4],
    terminal: bool,
    pub out: PlayerSet,
    children: Vec<Node>,
}

impl Node {
    pub fn new(board: Board, turn: Player, out: PlayerSet) -> Self {
        Node {
            board: board,
            turn: turn,
            visits: 0,
            //wins: [0, 0, 0, 0],
            wins: [0.0, 0.0, 0.0, 0.0],
            terminal: false,
            out: out,
            //out: PlayerSet::new(),
            children: Vec::new(),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn expand(&mut self) {
        if self.terminal {
            return;
        }

        for turn in Player::iter_from(self.turn) {
            if self.out.contains(turn) {
                continue;
            }

            let moves = self.board.find_moves(turn);
            if moves.is_empty() {
                self.out = self.out.add(self.turn);
            } else {
                self.turn = turn;
                for placement in moves {
                    let mut after = self.board.clone();
                    after.perform_placement(&placement, turn);
                    self.children.push(Node::new(after, turn.next(), self.out.clone()));
                }

                return;
            }
        }

        self.terminal = true;
    }

    fn select(&mut self) -> &mut Self {
        let root_visits = self.visits as f64;

        let utc = |child: &Node| {
            let wins = child.wins[self.turn as usize];

            let visits = (child.visits as f64) + EPSILON;
            let mean = wins / visits;
            mean + EXPLORATION * ((root_visits + 1.0).ln() / visits).sqrt()
        };

        let index = {
            let (i, _) = self
                .children
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| utc(a).partial_cmp(&utc(b)).unwrap())
                .unwrap();
            i
        };

        &mut self.children[index]
        //self.children.iter_mut().max_by(|a, b| utc(a.wins[self.turn as usize] as f32, a.visits as f32).partial_cmp(&utc(b.wins[self.turn as usize] as f32, b.visits as f32)).unwrap()).unwrap()
    }

    fn playout<R: Rng>(&mut self, rng: &mut R) -> [f64; 4] {
        self.visits += 1;

        let mut turn = self.turn;
        let mut board = self.board.clone();
        let mut out = PlayerSet::new();

        while !out.is_full() {
            if out.contains(turn) {
                turn = turn.next();
                continue;
            }

            if !board.play_randomly(turn, rng) {
                out = out.add(turn);
            }

            turn = turn.next();
        }

        let wins = self.board.find_wins();
        for turn in Player::iter() {
            self.wins[turn as usize] += wins[turn as usize];
        }

        wins
    }

    pub fn step<R: Rng>(&mut self, rng: &mut R) -> [f64; 4] {
        //self.visits += 1;

        if self.is_leaf() {
            self.expand();
        }

        /*
        if self.terminal {
            return if self.board.is_winner(self.turn) {
                true
            } else if self.board.is_winner(self.turn.opponent()) {
                false
            } else {
                rng.gen()
            };
        }
		*/

        if self.terminal {
            self.visits = 1;
            let wins = self.board.find_wins();
            /*
            for turn in player::iter() {
                self.wins[turn as usize] += wins[turn as usize];
            }*/

            return wins;
        }

        let child = self.select();
        let wins = if child.visits == 0 {
            child.playout(rng)
        } else {
            child.step(rng)
        };

        self.visits += 1;

        for turn in Player::iter() {
            self.wins[turn as usize] += wins[turn as usize];
        }

        wins
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }

    pub fn display(&self) {
        self.board.display();
    }

    pub fn best_child<R: Rng>(&self, rng: &mut R) -> &Self {
        print!("\x1b[s");
        print!("\x1b[20B");
        print!("\x1b[21C");

        let mut children = self.children.clone();
        children.sort_by(|a, b| b.visits.cmp(&a.visits));
        let mut last_visits: i32 = -1;
        let mut last_wins: f64 = -1.0;
        let mut counter = 1;
        let mut lines = 0;
        for child in children {
            if lines >= 18 {
                break;
            }

            let visits = child.visits as i32;
            let wins = child.wins[self.turn as usize];
            if visits == last_visits && wins == last_wins {
                counter += 1;
            } else {
                if !(last_visits == -1 && last_wins == -1.0) {
                    println!(
                        "{:.2}/{} = {:.1}% ({})",
                        last_wins,
                        last_visits,
                        100.0 * last_wins / (last_visits as f64),
                        counter
                    );
                    print!("\x1b[21C");
                    lines += 1;
                    //print!("\x1b[1B")
                }
                last_visits = child.visits as i32;
                last_wins = child.wins[self.turn as usize];
                counter = 1;
            }
        }

        println!(
            "{:.2}/{} = {:.1}% ({})",
            last_wins,
            last_visits,
            100.0 * last_wins / (last_visits as f64),
            counter
        );
        lines += 1;
        print!("\x1b[u");
        print!("\x1b[{}A", lines);

        let best_score = self
            .children
            .iter()
            .map(|n| (n.visits, n.wins[self.turn as usize]))
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap();
        let best: Vec<&Node> = self
            .children
            .iter()
            .filter(|n| (n.visits, n.wins[self.turn as usize]) == best_score)
            .collect();
        *rng.choose(&best).unwrap()
    }
}
