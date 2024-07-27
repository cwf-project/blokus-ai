use crate::player::*;

#[derive(Clone, Debug)]
pub struct PlayerSet {
    players: u8,
}

impl PlayerSet {
    pub fn new() -> Self {
        PlayerSet { players: 0 }
    }

    pub fn contains(&self, player: Player) -> bool {
        let shift = player as usize;
        ((self.players >> shift) & 1) == 1
    }

    pub fn add(&self, player: Player) -> PlayerSet {
        let mut after = self.clone();
        let shift = player as usize;
        after.players |= 1 << shift;
        after
    }

    pub fn is_full(&self) -> bool {
        self.players == 0xF
    }
}
