#[cfg_attr(rustfmt, rustfmt_skip)]
use crate::piece::*;
use crate::shape::Shape;
use std::slice;

pub const PIECES: [Piece; 21] = [
    Piece {
        orientations: &[Shape {
            bits: 0x00000000000000000000000000000001,
            attachments: &[0],
            width: 0,
            height: 0,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000000000000003,
                attachments: &[0, 1],
                width: 1,
                height: 0,
            },
            Shape {
                bits: 0x00000000000000000000000000100001,
                attachments: &[0, 20],
                width: 0,
                height: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000000000200003,
                attachments: &[0, 1, 21],
                width: 1,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000100003,
                attachments: &[0, 1, 20],
                width: 1,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000300002,
                attachments: &[1, 20, 21],
                width: 1,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000300001,
                attachments: &[0, 20, 21],
                width: 1,
                height: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000000000000007,
                attachments: &[0, 2],
                width: 2,
                height: 0,
            },
            Shape {
                bits: 0x00000000000000000000010000100001,
                attachments: &[0, 40],
                width: 0,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[Shape {
            bits: 0x00000000000000000000000000300003,
            attachments: &[0, 1, 20, 21],
            width: 1,
            height: 1,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000000000700002,
                attachments: &[1, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000200007,
                attachments: &[0, 2, 21],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000020000300002,
                attachments: &[1, 20, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000010000300001,
                attachments: &[0, 21, 40],
                width: 1,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000000000000000000000000F,
                attachments: &[0, 3],
                width: 3,
                height: 0,
            },
            Shape {
                bits: 0x00000000000000001000010000100001,
                attachments: &[0, 60],
                width: 0,
                height: 3,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000000000700004,
                attachments: &[2, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000700001,
                attachments: &[0, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000400007,
                attachments: &[0, 2, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000100007,
                attachments: &[0, 2, 20],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000030000200002,
                attachments: &[1, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000030000100001,
                attachments: &[0, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000020000200003,
                attachments: &[0, 1, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000010000100003,
                attachments: &[0, 1, 40],
                width: 1,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000000000300006,
                attachments: &[1, 2, 20, 21],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000600003,
                attachments: &[0, 1, 21, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000010000300002,
                attachments: &[1, 20, 21, 40],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000020000300001,
                attachments: &[0, 20, 21, 41],
                width: 1,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000000000F00001,
                attachments: &[0, 20, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000F00008,
                attachments: &[3, 20, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x0000000000000000000000000010000F,
                attachments: &[0, 3, 20],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x0000000000000000000000000080000F,
                attachments: &[0, 3, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000002000020000200003,
                attachments: &[0, 1, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000001000010000100003,
                attachments: &[0, 1, 60],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000003000020000200002,
                attachments: &[1, 60, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000003000010000100001,
                attachments: &[0, 60, 61],
                width: 1,
                height: 3,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000070000200002,
                attachments: &[1, 40, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000020000200007,
                attachments: &[0, 2, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000040000700004,
                attachments: &[2, 20, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000010000700001,
                attachments: &[0, 22, 40],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000070000100001,
                attachments: &[0, 40, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000070000400004,
                attachments: &[2, 40, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000010000100007,
                attachments: &[0, 2, 40],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000040000400007,
                attachments: &[0, 2, 42],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000000000000000000030000E,
                attachments: &[1, 3, 20, 21],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000C00007,
                attachments: &[0, 2, 22, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000E00003,
                attachments: &[0, 1, 21, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x0000000000000000000000000070000C,
                attachments: &[2, 3, 20, 22],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000001000010000300002,
                attachments: &[1, 20, 21, 60],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000002000020000300001,
                attachments: &[0, 20, 21, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000002000030000100001,
                attachments: &[0, 40, 41, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000001000030000200002,
                attachments: &[1, 40, 41, 60],
                width: 1,
                height: 3,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000010000700004,
                attachments: &[2, 20, 22, 40],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000040000700001,
                attachments: &[0, 20, 22, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000030000200006,
                attachments: &[1, 2, 40, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000060000200003,
                attachments: &[0, 1, 41, 42],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000000000000000000000001F,
                attachments: &[0, 4],
                width: 4,
                height: 0,
            },
            Shape {
                bits: 0x00000000000100001000010000100001,
                attachments: &[0, 80],
                width: 0,
                height: 4,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000030000300001,
                attachments: &[0, 21, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000030000300002,
                attachments: &[1, 20, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000010000300003,
                attachments: &[0, 1, 21, 40],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000020000300003,
                attachments: &[0, 1, 20, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000000000600007,
                attachments: &[0, 2, 21, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000300007,
                attachments: &[0, 2, 20, 21],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000700006,
                attachments: &[1, 2, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000700003,
                attachments: &[0, 1, 20, 22],
                width: 2,
                height: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000010000300006,
                attachments: &[1, 2, 20, 21, 40],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000040000600003,
                attachments: &[0, 1, 21, 22, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000060000300001,
                attachments: &[0, 20, 21, 41, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000030000600004,
                attachments: &[2, 21, 22, 40, 41],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000030000100003,
                attachments: &[1, 1, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000030000200003,
                attachments: &[0, 1, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000000000500007,
                attachments: &[0, 2, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000700005,
                attachments: &[0, 2, 20, 22],
                width: 2,
                height: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000020000300006,
                attachments: &[1, 2, 20, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000020000600003,
                attachments: &[0, 1, 22, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000060000300002,
                attachments: &[1, 20, 41, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000030000600002,
                attachments: &[1, 22, 40, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000010000700002,
                attachments: &[1, 20, 22, 40],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000040000700002,
                attachments: &[1, 20, 22, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000020000700001,
                attachments: &[0, 20, 22, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: 0x00000000000000000000020000700004,
                attachments: &[2, 20, 22, 41],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[Shape {
            bits: 0x00000000000000000000020000700002,
            attachments: &[1, 20, 22, 41],
            width: 2,
            height: 2,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x00000000000000000000000000F00002,
                attachments: &[1, 20, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000000000000000F00004,
                attachments: &[2, 20, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x0000000000000000000000000020000F,
                attachments: &[0, 3, 21],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x0000000000000000000000000040000F,
                attachments: &[0, 3, 22],
                width: 3,
                height: 1,
            },
            Shape {
                bits: 0x00000000000000002000020000300002,
                attachments: &[1, 20, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000001000010000300001,
                attachments: &[0, 21, 60],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000002000030000200002,
                attachments: &[1, 40, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: 0x00000000000000001000030000100001,
                attachments: &[0, 41, 60],
                width: 1,
                height: 3,
            },
        ],
    },
];

/*
pub const PIECES: [Piece; 21] = [
    Piece {
        orientations: &[Shape {
            bits: [1, 0],
            attachments: &[0],
            width: 0,
            height: 0,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [3, 0],
                attachments: &[0, 1],
                width: 1,
                height: 0,
            },
            Shape {
                bits: [1048577, 0],
                attachments: &[0, 20],
                width: 0,
                height: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [2097155, 0],
                attachments: &[0, 1, 21],
                width: 1,
                height: 1,
            },
            Shape {
                bits: [1048579, 0],
                attachments: &[0, 1, 20],
                width: 1,
                height: 1,
            },
            Shape {
                bits: [3145730, 0],
                attachments: &[1, 20, 21],
                width: 1,
                height: 1,
            },
            Shape {
                bits: [3145729, 0],
                attachments: &[0, 20, 21],
                width: 1,
                height: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [7, 0],
                attachments: &[0, 2],
                width: 2,
                height: 0,
            },
            Shape {
                bits: [1099512676353, 0],
                attachments: &[0, 40],
                width: 0,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[Shape {
            bits: [3145731, 0],
            attachments: &[0, 1, 20, 21],
            width: 1,
            height: 1,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [7340034, 0],
                attachments: &[1, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [2097159, 0],
                attachments: &[0, 2, 21],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [2199026401282, 0],
                attachments: &[1, 20, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [1099514773505, 0],
                attachments: &[0, 21, 40],
                width: 1,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [15, 0],
                attachments: &[0, 3],
                width: 3,
                height: 0,
            },
            Shape {
                bits: [1099512676353, 1],
                attachments: &[0, 60],
                width: 0,
                height: 3,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [7340036, 0],
                attachments: &[2, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [7340033, 0],
                attachments: &[0, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [4194311, 0],
                attachments: &[0, 2, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [1048583, 0],
                attachments: &[0, 2, 20],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [3298536980482, 0],
                attachments: &[1, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [3298535931905, 0],
                attachments: &[0, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [2199025352707, 0],
                attachments: &[0, 1, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [1099512676355, 0],
                attachments: &[0, 1, 40],
                width: 1,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [3145734, 0],
                attachments: &[1, 2, 20, 21],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [6291459, 0],
                attachments: &[0, 1, 21, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [1099514773506, 0],
                attachments: &[1, 20, 21, 40],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [2199026401281, 0],
                attachments: &[0, 20, 21, 41],
                width: 1,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [15728641, 0],
                attachments: &[0, 20, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [15728648, 0],
                attachments: &[3, 20, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [1048591, 0],
                attachments: &[0, 3, 20],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [8388623, 0],
                attachments: &[0, 3, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [2199025352707, 2],
                attachments: &[0, 1, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [1099512676355, 1],
                attachments: &[0, 1, 60],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [2199025352706, 3],
                attachments: &[1, 60, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [1099512676353, 3],
                attachments: &[0, 60, 61],
                width: 1,
                height: 3,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [7696583491586, 0],
                attachments: &[1, 40, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [2199025352711, 0],
                attachments: &[0, 2, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [4398053851140, 0],
                attachments: &[2, 20, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [1099518967809, 0],
                attachments: &[0, 22, 40],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [7696582443009, 0],
                attachments: &[0, 40, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [7696585588740, 0],
                attachments: &[2, 40, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [1099512676359, 0],
                attachments: &[0, 2, 40],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [4398050705415, 0],
                attachments: &[0, 2, 42],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [3145742, 0],
                attachments: &[1, 3, 20, 21],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [12582919, 0],
                attachments: &[0, 2, 22, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [14680067, 0],
                attachments: &[0, 1, 21, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [7340044, 0],
                attachments: &[2, 3, 20, 22],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [1099514773506, 1],
                attachments: &[1, 20, 21, 60],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [2199026401281, 2],
                attachments: &[0, 20, 21, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [3298535931905, 2],
                attachments: &[0, 40, 41, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [3298536980482, 1],
                attachments: &[1, 40, 41, 60],
                width: 1,
                height: 3,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [1099518967812, 0],
                attachments: &[2, 20, 22, 40],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [4398053851137, 0],
                attachments: &[0, 20, 22, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [3298536980486, 0],
                attachments: &[1, 2, 40, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [6597071863811, 0],
                attachments: &[0, 1, 41, 42],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [31, 0],
                attachments: &[0, 4],
                width: 4,
                height: 0,
            },
            Shape {
                bits: [1099512676353, 1048577],
                attachments: &[0, 80],
                width: 0,
                height: 4,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [3298538029057, 0],
                attachments: &[0, 21, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [3298538029058, 0],
                attachments: &[1, 20, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [1099514773507, 0],
                attachments: &[0, 1, 21, 40],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [2199026401283, 0],
                attachments: &[0, 1, 20, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [6291463, 0],
                attachments: &[0, 2, 21, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [3145735, 0],
                attachments: &[0, 2, 20, 21],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [7340038, 0],
                attachments: &[1, 2, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [7340035, 0],
                attachments: &[0, 1, 20, 22],
                width: 2,
                height: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [1099514773510, 0],
                attachments: &[1, 2, 20, 21, 40],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [4398052802563, 0],
                attachments: &[0, 1, 21, 22, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [6597072912385, 0],
                attachments: &[0, 20, 21, 41, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [3298541174788, 0],
                attachments: &[2, 21, 22, 40, 41],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [3298535931907, 0],
                attachments: &[0, 1, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [3298536980483, 0],
                attachments: &[0, 1, 40, 41],
                width: 1,
                height: 2,
            },
            Shape {
                bits: [5242887, 0],
                attachments: &[0, 2, 20, 22],
                width: 2,
                height: 1,
            },
            Shape {
                bits: [7340037, 0],
                attachments: &[0, 2, 20, 22],
                width: 2,
                height: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [2199026401286, 0],
                attachments: &[1, 2, 20, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [2199029547011, 0],
                attachments: &[0, 1, 22, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [6597072912386, 0],
                attachments: &[1, 20, 41, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [3298541174786, 0],
                attachments: &[1, 22, 40, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [1099518967810, 0],
                attachments: &[1, 20, 22, 40],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [4398053851138, 0],
                attachments: &[1, 20, 22, 42],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [2199030595585, 0],
                attachments: &[0, 20, 22, 41],
                width: 2,
                height: 2,
            },
            Shape {
                bits: [2199030595588, 0],
                attachments: &[2, 20, 22, 41],
                width: 2,
                height: 2,
            },
        ],
    },
    Piece {
        orientations: &[Shape {
            bits: [2199030595586, 0],
            attachments: &[1, 20, 22, 41],
            width: 2,
            height: 2,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: [15728642, 0],
                attachments: &[1, 20, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [15728644, 0],
                attachments: &[2, 20, 23],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [2097167, 0],
                attachments: &[0, 3, 21],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [4194319, 0],
                attachments: &[0, 3, 22],
                width: 3,
                height: 1,
            },
            Shape {
                bits: [2199026401282, 2],
                attachments: &[1, 20, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [1099514773505, 1],
                attachments: &[0, 21, 60],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [3298536980482, 2],
                attachments: &[1, 40, 61],
                width: 1,
                height: 3,
            },
            Shape {
                bits: [3298535931905, 1],
                attachments: &[0, 41, 60],
                width: 1,
                height: 3,
            },
        ],
    },
];
*/

pub fn iter() -> slice::Iter<'static, Piece> {
    PIECES.iter()
}

pub fn by_id(id: PieceId) -> &'static Piece {
    unsafe { PIECES.get_unchecked(id as usize) }
}
