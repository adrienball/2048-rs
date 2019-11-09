use std::fmt::{Debug, Display, Formatter};

/// `Board` is the main object of the 2048 game. It represents the state of the 16 tiles.
///
/// The representation relies on a single u64 value which encode the 16 values by leveraging the
/// fact that each value is a power of 2. This allows to represent values from `0` to `2^15`.
///
/// As an example, to encode `32` we take its binary decomposition which is `2^5`. Then, the binary
/// representation of `5` is computed, and is `"101"`. To make sure that all 16 representations
/// have 4 bits, a prefix consisting of 0s is added: `"101"` is transformed into `"0101"`.
///
/// `0` is actually not a power of 2, hence the previous example cannot be applied to find its
/// representation. However, an important detail is that the specific value `1` is not part of
/// the game, its representation would have been `"0000"`. This value is thus the one we use to
/// represent `0`.
///
/// # Examples
///
/// ```
/// use crate::Board;
///
/// let board_values: Vec<u16> = vec![
///     0, 0, 0, 0,
///     0, 0, 0, 0,
///     0, 0, 4, 0,
///     0, 16, 0, 8,
/// ];
///
/// let board = Board::try_from(board_values).unwrap();
/// let board_repr: u64 = 2u64.pow(0 + 0) + 2u64.pow(1 + 0) + 2u64.pow(2 + 8) + 2u64.pow(1 + 20);
/// assert_eq!(board_repr, board.state);
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Board {
    state: u64,
}

/// The four directions in which the tiles can be moved
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn all() -> &'static [Direction; 4] {
        &[
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
    }
}

impl Board {
    /// Returns the value at the corresponding index
    /// The underlying vector representation is used here
    pub fn get_value(&self, tile_idx: u8) -> u16 {
        let exponent = (self.state >> 4 * (16 - tile_idx as u64 - 1)) & 15;
        if exponent == 0 {
            return 0;
        }
        2 << (exponent - 1) as u16
    }

    /// Sets the value `tile_value` at the index `tile_idx`
    pub fn set_value(self, tile_idx: u8, tile_value: u16) -> Self {
        let mut board_vec: Vec<_> = self.into();
        board_vec[tile_idx as usize] = tile_value;
        board_vec.into()
    }

    /// Returns the maximum value of the board
    pub fn max_value(self) -> u16 {
        let board_vec: Vec<_> = self.into();
        board_vec.into_iter().max().unwrap()
    }

    /// Returns the indices of empty tiles
    pub fn empty_tiles_indices(self) -> Vec<u8> {
        Vec::from(self)
            .into_iter()
            .enumerate()
            .filter_map(|(idx, tile)| if tile == 0 { Some(idx as u8) } else { None })
            .collect()
    }

    /// Moves the tiles in the provided `Direction` and returns the resulting `Board`
    pub fn move_to(self, direction: Direction) -> Self {
        match direction {
            Direction::Left => self.into_left(),
            Direction::Right => self.into_right(),
            Direction::Up => self.into_up(),
            Direction::Down => self.into_down(),
        }
    }

    fn into_left(self) -> Self {
        let mut board_vec: Vec<_> = self.into();
        for row in 0..4 {
            // we can't use -1 here so we use 1 as it never appears in a board
            let mut prev_value = 1;
            let mut new_value_idx = 4 * row;
            let mut col = 0;
            // whether or not cells have been moved in this row
            let mut moved = false;
            while col < 4 {
                let value_idx = 4 * row + col;
                let value = board_vec[value_idx];
                if value == 0 {
                    col += 1;
                    moved = true;
                    continue;
                } else if value == prev_value {
                    board_vec[new_value_idx - 1] <<= 1;
                    board_vec[value_idx] = 0;
                    prev_value = 1;
                    moved = true;
                } else {
                    board_vec[new_value_idx] = value;
                    if moved {
                        board_vec[value_idx] = 0;
                    }
                    prev_value = value;
                    new_value_idx += 1;
                }
                col += 1;
            }
        }
        board_vec.into()
    }

    fn into_right(self) -> Self {
        let mut board_vec: Vec<_> = self.into();
        for row in 0..4 {
            // we can't use -1 here so we use 1 as it never appears in a board
            let mut prev_value = 1;
            let mut new_value_idx = 4 * row + 3;
            let mut col = 3;
            // whether or not cells have been moved in this row
            let mut moved = false;
            loop {
                let value_idx = 4 * row + col;
                let value = board_vec[value_idx];
                if value == 0 {
                    if col == 0 {
                        break;
                    }
                    col -= 1;
                    moved = true;
                    continue;
                } else if value == prev_value {
                    board_vec[new_value_idx + 1] <<= 1;
                    board_vec[value_idx] = 0;
                    prev_value = 1;
                    moved = true;
                } else {
                    board_vec[new_value_idx] = value;
                    if moved {
                        board_vec[value_idx] = 0;
                    }
                    prev_value = value;
                    if col > 0 {
                        new_value_idx -= 1;
                    }
                }
                if col == 0 {
                    break;
                }
                col -= 1;
            }
        }
        board_vec.into()
    }

    fn into_up(self) -> Self {
        let mut board_vec: Vec<_> = self.into();
        for col in 0..4 {
            // we can't use -1 here so we use 1 as it never appears in a board
            let mut prev_value = 1;
            let mut new_value_idx = col;
            let mut row = 0;
            // whether or not cells have been moved in this row
            let mut moved = false;
            while row < 4 {
                let value_idx = 4 * row + col;
                let value = board_vec[value_idx];
                if value == 0 {
                    row += 1;
                    moved = true;
                    continue;
                } else if value == prev_value {
                    board_vec[new_value_idx - 4] <<= 1;
                    board_vec[value_idx] = 0;
                    prev_value = 1;
                    moved = true;
                } else {
                    board_vec[new_value_idx] = value;
                    if moved {
                        board_vec[value_idx] = 0;
                    }
                    prev_value = value;
                    new_value_idx += 4;
                }
                row += 1;
            }
        }
        board_vec.into()
    }

    fn into_down(self) -> Self {
        let mut board_vec: Vec<_> = self.into();
        for col in 0..4 {
            // we can't use -1 here so we use 1 as it never appears in a board
            let mut prev_value = 1;
            let mut new_value_idx = 4 * 3 + col;
            let mut row = 3;
            // whether or not cells have been moved in this row
            let mut moved = false;
            loop {
                let value_idx = 4 * row + col;
                let value = board_vec[value_idx];
                if value == 0 {
                    if row == 0 {
                        break;
                    }
                    row -= 1;
                    moved = true;
                    continue;
                } else if value == prev_value {
                    board_vec[new_value_idx + 4] <<= 1;
                    board_vec[value_idx] = 0;
                    prev_value = 1;
                    moved = true;
                } else {
                    board_vec[new_value_idx] = value;
                    if moved {
                        board_vec[value_idx] = 0;
                    }
                    prev_value = value;
                    if row > 0 {
                        new_value_idx -= 4;
                    }
                }
                if row == 0 {
                    break;
                }
                row -= 1;
            }
        }
        board_vec.into()
    }
}

impl IntoIterator for Board {
    type Item = u16;
    type IntoIter = BoardIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoardIntoIterator {
            board: self,
            index: 0,
        }
    }
}

struct BoardIntoIterator {
    board: Board,
    index: u8,
}

impl Iterator for BoardIntoIterator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            16 => None,
            _ => Some(self.board.get_value(self.index)),
        }
    }
}

impl From<Vec<u16>> for Board {
    fn from(tiles: Vec<u16>) -> Self {
        let mut state: u64 = 0;
        for tile_value in tiles.into_iter() {
            state <<= 4;
            if tile_value == 0 {
                continue;
            }
            let mut exponent = 1;
            let mut v = tile_value >> 2;
            while v != 0 {
                v = v >> 1;
                exponent += 1;
            }
            state |= exponent;
        }
        Self { state }
    }
}

impl From<Board> for Vec<u16> {
    fn from(board: Board) -> Self {
        (0..16)
            .into_iter()
            .map(|tile_idx| board.get_value(tile_idx))
            .collect()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let tiles: Vec<u16> = self.clone().into();
        let mut display = String::new();
        display.push_str("\n+-------+-------+-------+-------+\n");
        for (i, tile) in tiles.into_iter().enumerate() {
            display.push_str(&*format!("| {}\t", tile));
            if tile < 10 {
                display.push_str("\t");
            }
            if i % 4 == 3 {
                display.push_str("|\n");
                display.push_str("+-------+-------+-------+-------+\n");
            }
        }
        write!(f, "{}", display)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_vec_to_board() {
        // Given
        #[rustfmt::skip]
        let vec_board: Vec<u16> = vec![
            0, 2, 0, 0,
            32768, 0, 0, 2,
            0, 0, 16, 4,
            8, 2, 16, 64
        ];

        // When
        let board = Board::from(vec_board.clone());

        // Then
        let into_vec_board: Vec<u16> = board.into();
        assert_eq!(vec_board, into_vec_board);
    }

    #[test]
    fn should_use_binary_representation() {
        // Given
        #[rustfmt::skip]
        let board_values = vec![
            0, 0, 0, 0, 
            0, 0, 0, 0, 
            0, 0, 4, 0, 
            0, 16, 0, 8
        ];

        // When
        let board = Board::from(board_values);

        // Then
        let board_repr: u64 =
            2u64.pow(0 + 0) + 2u64.pow(1 + 0) + 2u64.pow(2 + 8) + 2u64.pow(1 + 20);
        assert_eq!(board_repr as u64, board.state);
    }

    #[test]
    fn should_move_left() {
        // Given
        #[rustfmt::skip]
        let board = Board::from(vec![
            0, 0, 0, 2,
            2, 2, 4, 0,
            4, 2, 8, 512,
            16, 16, 32, 32,
        ]);

        // When
        let left_board = board.into_left();

        // Then
        #[rustfmt::skip]
        let expected_board = Board::from(vec![
            2, 0, 0, 0,
            4, 4, 0, 0,
            4, 2, 8, 512,
            32, 64, 0, 0,
        ]);
        assert_eq!(expected_board, left_board);
    }

    #[test]
    fn should_move_right() {
        // Given
        #[rustfmt::skip]
        let board = Board::from(vec![
            512, 8, 2, 4,
            2, 0, 0, 0,
            0, 4, 2, 2,
            32, 32, 16, 16,
        ]);

        // When
        let right_board = board.into_right();

        // Then
        #[rustfmt::skip]
        let expected_board = Board::from(vec![
            512, 8, 2, 4,
            0, 0, 0, 2,
            0, 0, 4, 4,
            0, 0, 64, 32,
        ]);
        assert_eq!(expected_board, right_board);
    }

    #[test]
    fn should_move_up() {
        // Given
        #[rustfmt::skip]
        let board = Board::from(vec![
            0, 2, 512, 16,
            0, 2, 8, 16,
            0, 4, 2, 32,
            2, 0, 4, 32,
        ]);

        // When
        let up_board = board.into_up();

        // Then
        #[rustfmt::skip]
        let expected_board = Board::from(vec![
            2, 4, 512, 32,
            0, 4, 8, 64,
            0, 0, 2, 0,
            0, 0, 4, 0,
        ]);
        assert_eq!(expected_board, up_board);
    }

    #[test]
    fn should_move_down() {
        // Given
        #[rustfmt::skip]
        let board = Board::from(vec![
            2, 0, 512, 32,
            0, 4, 8, 32,
            0, 2, 2, 16,
            0, 2, 4, 16,
        ]);

        // When
        let down_board = board.into_down();

        // Then
        #[rustfmt::skip]
        let expected_board = Board::from(vec![
            0, 0, 512, 0,
            0, 0, 8, 0,
            0, 4, 2, 64,
            2, 4, 4, 32,
        ]);
        assert_eq!(expected_board, down_board);
    }

    #[test]
    fn should_get_max_value() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            0, 2, 0, 2048,
            0, 256, 0, 512,
            0, 0, 1024, 4,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board);

        // When
        let max_value = board.max_value();

        // Then
        assert_eq!(2048, max_value);
    }

    #[test]
    fn should_get_empty_tiles() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            0, 2, 0, 2048,
            0, 256, 0, 512,
            0, 0, 1024, 4,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board);

        // When
        let empty_tiles = board.empty_tiles_indices();
        assert_eq!(vec![0, 2, 4, 6, 8, 9], empty_tiles);
    }

    #[test]
    fn should_display_board() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            0, 2, 0, 32768,
            0, 256, 0, 512,
            0, 0, 1024, 4,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board);

        // When
        let display = format!("{}", board);

        // Then
        let expected_display = r#"
+-------+-------+-------+-------+
| 0		| 2		| 0		| 32768	|
+-------+-------+-------+-------+
| 0		| 256	| 0		| 512	|
+-------+-------+-------+-------+
| 0		| 0		| 1024	| 4		|
+-------+-------+-------+-------+
| 8		| 2		| 16	| 64	|
+-------+-------+-------+-------+
"#;
        assert_eq!(expected_display, display);
    }
}
