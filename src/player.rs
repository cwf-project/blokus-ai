use rand::{distributions::*, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    Blue = 0,
    Yellow = 1,
    Red = 2,
    Green = 3,
}

impl Player {
    pub fn next(self) -> Self {
        match self {
            Player::Blue => Player::Yellow,
            Player::Yellow => Player::Red,
            Player::Red => Player::Green,
            Player::Green => Player::Blue,
        }
    }

    pub fn iter_from(start: Player) -> PlayerIterator {
        PlayerIterator::new(start)
    }

    pub fn iter() -> PlayerIterator {
        PlayerIterator::new(Player::Blue)
    }
}

pub struct PlayerIterator {
    turn: Player,
    count: u8,
}

impl PlayerIterator {
    fn new(start: Player) -> Self {
        PlayerIterator {
            turn: start,
            count: 0,
        }
    }
}

impl Iterator for PlayerIterator {
    type Item = Player;

    fn next(&mut self) -> Option<Player> {
        if self.count == 4 {
            return None;
        }

        let turn = self.turn;
        self.turn = self.turn.next();
        self.count += 1;

        Some(turn)
    }
}


impl Distribution<Player> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Player {
        *rng.choose(&[Player::Blue, Player::Yellow, Player::Red, Player::Green])
            .unwrap()
    }
}
