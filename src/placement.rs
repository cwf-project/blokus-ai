pub struct Placement {
    block: u8,
    shift: u8,
    piece: u8,
    orientation: u8,
}

impl Placement {
    pub fn new(block: usize, shift: usize, piece: usize, orientation: usize) -> Self {
        Placement {
            block: block as u8,
            shift: shift as u8,
            piece: piece as u8,
            orientation: orientation as u8,
        }
    }

    pub fn block(&self) -> usize {
        self.block as usize
    }

    pub fn shift(&self) -> usize {
        self.shift as usize
    }

    pub fn piece(&self) -> usize {
        self.piece as usize
    }

    pub fn orientation(&self) -> usize {
        self.orientation as usize
    }
}
