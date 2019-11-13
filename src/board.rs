use crate::utils::{build_left_moves_table, build_right_moves_table, get_exponent};
use lazy_static::lazy_static;
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
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
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

lazy_static! {
    static ref LEFT_MOVES_TABLE: Vec<u16> = build_left_moves_table();
    static ref RIGHT_MOVES_TABLE: Vec<u16> = build_right_moves_table();
}

impl Board {
    /// Returns the value at the corresponding index
    /// The underlying vector representation is used here
    pub fn get_value(&self, tile_idx: u8) -> u16 {
        let exponent = self.get_exponent_value(tile_idx);
        if exponent == 0 {
            return 0;
        }
        2 << (exponent - 1) as u16
    }

    /// Returns the exponent of the value at the corresponding index
    /// For example, if `get_value(3)` returns `512`, then `get_exponent_value(3)` will return `9`
    /// because 512 = 2^9
    pub fn get_exponent_value(&self, tile_idx: u8) -> u8 {
        ((self.state >> 4 * (15 - tile_idx as u64)) & 0b1111) as u8
    }

    /// Sets the value `tile_value` at the index `tile_idx`
    pub fn set_value(self, tile_idx: u8, tile_value: u16) -> Self {
        let exponent = get_exponent(tile_value);
        self.set_value_by_exponent(tile_idx, exponent)
    }

    /// Sets the value `tile_value` at the index `tile_idx` by specifying the exponent directly.
    /// For example, `set_value_by_exponent(3, 9)` is equivalent to `set_value(3, 512)`
    /// because 512 = 2^9
    pub fn set_value_by_exponent(self, tile_idx: u8, value_exponent: u64) -> Self {
        let bits_shift = ((15 - tile_idx) * 4) as u64;
        // bitmask with 0000 at the corresponding tile_idx and 1s everywhere else
        let clear_mask = !(0b1111 << bits_shift);
        let update_mask = value_exponent << bits_shift;
        let new_state = (self.state & clear_mask) | update_mask;
        Board { state: new_state }
    }

    /// Returns the 16 bits of the the specified row
    pub fn get_row(&self, row_idx: u8) -> u16 {
        let bit_shift = ((3 - row_idx) * 16) as u64;
        ((self.state & (0b1111111111111111u64 << bit_shift)) >> bit_shift) as u16
    }

    /// Returns the 16 bits of the the specified column
    pub fn get_column(&self, col_idx: u8) -> u16 {
        let col_shift: u64 = 4 * (3 - col_idx as u64);
        let mut column = (self.state >> col_shift) & 0b1111;
        column |= ((self.state >> (col_shift + 16)) << 4) & 0b11110000;
        column |= ((self.state >> (col_shift + 32)) << 8) & 0b111100000000;
        column |= ((self.state >> (col_shift + 48)) << 12) & 0b1111000000000000;
        column as u16
    }

    /// Returns the maximum value of the board
    pub fn max_value(self) -> u16 {
        let exponent = self.into_iter().max().unwrap();
        2 << (exponent - 1) as u16
    }

    /// Returns the indices of empty tiles
    pub fn empty_tiles_indices(self) -> Vec<u8> {
        let mut indices = Vec::<u8>::with_capacity(16);
        let mut i = 0;
        for tile in self.into_iter() {
            if tile == 0 {
                indices.push(i)
            }
            i += 1;
        }
        indices
    }

    /// Returns the number of distinct tiles, excluding empty tiles
    pub fn count_distinct_tiles(self) -> usize {
        let mut bitset: u16 = 0;
        let mut state = self.state;
        while state != 0 {
            bitset |= 1 << (state & 0b1111) as u16;
            state >>= 4;
        }
        // exclude empty tiles from the count
        bitset >>= 1;
        let mut count: usize = 0;
        while bitset != 0 {
            bitset &= bitset - 1;
            count += 1;
        }
        count
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
        let mut state: u64 = 0;
        for row_idx in 0..4 {
            let row = self.get_row(row_idx);
            state |= (LEFT_MOVES_TABLE[row as usize] as u64) << (16 * (3 - row_idx) as u64);
        }
        Self { state }
    }

    fn into_right(self) -> Self {
        let mut state: u64 = 0;
        for row_idx in 0..4 {
            let row = self.get_row(row_idx);
            state |= (RIGHT_MOVES_TABLE[row as usize] as u64) << (16 * (3 - row_idx) as u64);
        }
        Self { state }
    }

    fn into_up(self) -> Self {
        let mut state = 0;
        for col_idx in 0..4 {
            let col = self.get_column(col_idx);
            let up_col = LEFT_MOVES_TABLE[col as usize] as u64;
            let col_shift = 4 * (3 - col_idx) as u64;
            state |= (up_col & 0b1111000000000000) << 36 + col_shift;
            state |= (up_col & 0b111100000000) << 24 + col_shift;
            state |= (up_col & 0b11110000) << 12 + col_shift;
            state |= (up_col & 0b1111) << col_shift;
        }
        Self { state }
    }

    fn into_down(self) -> Self {
        let mut state = 0;
        for col_idx in 0..4 {
            let col = self.get_column(col_idx);
            let up_col = RIGHT_MOVES_TABLE[col as usize] as u64;
            let col_shift = 4 * (3 - col_idx) as u64;
            state |= (up_col & 0b1111000000000000) << 36 + col_shift;
            state |= (up_col & 0b111100000000) << 24 + col_shift;
            state |= (up_col & 0b11110000) << 12 + col_shift;
            state |= (up_col & 0b1111) << col_shift;
        }
        Self { state }
    }
}

impl IntoIterator for Board {
    type Item = u8;
    type IntoIter = BoardIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoardIntoIterator {
            state: self.state,
            index: 0,
        }
    }
}

pub struct BoardIntoIterator {
    state: u64,
    index: u8,
}

impl Iterator for BoardIntoIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            16 => None,
            _ => {
                let exponent = self.state >> 60;
                self.state <<= 4;
                self.index += 1;
                Some(exponent as u8)
            }
        }
    }
}

impl Board {
    pub fn into_reversed_iter(self) -> BoardIntoReversedIterator {
        BoardIntoReversedIterator {
            state: self.state,
            index: 0,
        }
    }
}

pub struct BoardIntoReversedIterator {
    state: u64,
    index: u8,
}

impl Iterator for BoardIntoReversedIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            16 => None,
            _ => {
                let exponent = self.state & 0b1111;
                self.state >>= 4;
                self.index += 1;
                Some(exponent as u8)
            }
        }
    }
}

impl From<Vec<u16>> for Board {
    fn from(tiles: Vec<u16>) -> Self {
        let mut state: u64 = 0;
        for tile_value in tiles.into_iter() {
            state <<= 4;
            state |= get_exponent(tile_value);
        }
        Self { state }
    }
}

impl From<Board> for Vec<u16> {
    fn from(board: Board) -> Self {
        board
            .into_iter()
            .map(|tile_exponent| {
                if tile_exponent == 0 {
                    0
                } else {
                    2 << (tile_exponent - 1) as u16
                }
            })
            .collect()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut display = String::new();
        display.push_str("\n+-------+-------+-------+-------+\n");
        for (i, tile) in Vec::from(*self).into_iter().enumerate() {
            display.push_str(&*format!("| {}", tile));
            if tile < 10 {
                display.push_str("     ");
            } else if tile < 100 {
                display.push_str("    ");
            } else if tile < 1000 {
                display.push_str("   ");
            } else if tile < 10000 {
                display.push_str("  ");
            } else {
                display.push_str(" ");
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
    fn should_iterate_over_exponents() {
        // Given
        #[rustfmt::skip]
        let vec_board: Vec<u16> = vec![
            0, 2, 0, 0,
            32768, 0, 0, 2,
            0, 0, 16, 4,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board.clone());

        // When
        let exponents: Vec<_> = board.into_iter().collect();

        // Then
        #[rustfmt::skip]
        let expected_exponents = vec![
            0, 1, 0, 0,
            15, 0, 0, 1,
            0, 0, 4, 2,
            3, 1, 4, 6
        ];
        assert_eq!(expected_exponents, exponents);
    }

    #[test]
    fn should_reverse_iterate_over_exponents() {
        // Given
        #[rustfmt::skip]
        let vec_board: Vec<u16> = vec![
            0, 2, 0, 0,
            32768, 0, 0, 2,
            0, 0, 16, 4,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board.clone());

        // When
        let exponents: Vec<_> = board.into_reversed_iter().collect();

        // Then
        #[rustfmt::skip]
         let mut expected_exponents = vec![
            0, 1, 0, 0,
            15, 0, 0, 1,
            0, 0, 4, 2,
            3, 1, 4, 6
        ];
        expected_exponents.reverse();
        assert_eq!(expected_exponents, exponents);
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
    fn should_get_value() {
        // Given
        #[rustfmt::skip]
         let board = Board::from(vec![
            0, 4, 0, 2,
            2, 0, 4, 0,
            4, 2, 0, 512,
            16, 8, 32, 32,
        ]);

        // When / Then
        assert_eq!(512, board.get_value(11));
    }

    #[test]
    fn should_get_exponent_value() {
        // Given
        #[rustfmt::skip]
            let board = Board::from(vec![
            0, 4, 0, 2,
            2, 0, 4, 0,
            4, 2, 0, 512,
            16, 8, 32, 32,
        ]);

        // When / Then
        assert_eq!(9, board.get_exponent_value(11));
    }

    #[test]
    fn should_get_row() {
        // Given
        #[rustfmt::skip]
        let board = Board::from(vec![
            0, 4, 0, 2,
            2, 0, 4, 0,
            4, 2, 0, 512,
            16, 8, 32, 32,
        ]);

        // When / Then
        assert_eq!(0b0010000100001001, board.get_row(2));
    }

    #[test]
    fn should_get_column() {
        // Given
        #[rustfmt::skip]
            let board = Board::from(vec![
            0, 4, 0, 2,
            2, 0, 512, 0,
            4, 2, 2, 512,
            16, 8, 32, 32,
        ]);

        // When / Then
        assert_eq!(0b0000100100010101, board.get_column(2));
    }

    #[test]
    fn should_set_value() {
        // Given
        #[rustfmt::skip]
        let board = Board::from(vec![
            0, 4, 0, 2,
            2, 0, 4, 0,
            4, 2, 0, 512,
            16, 8, 32, 32,
        ]);

        // When
        let board = board.set_value(5, 32).set_value(8, 64);

        // Then
        #[rustfmt::skip]
        let expected_board = Board::from(vec![
            0, 4, 0, 2,
            2, 32, 4, 0,
            64, 2, 0, 512,
            16, 8, 32, 32,
        ]);
        assert_eq!(expected_board, board);
    }

    #[test]
    fn should_set_value_by_exponent() {
        // Given
        #[rustfmt::skip]
            let board = Board::from(vec![
            0, 4, 0, 2,
            2, 0, 4, 0,
            4, 2, 0, 512,
            16, 8, 32, 32,
        ]);

        // When
        let board = board
            .set_value_by_exponent(5, 5)
            .set_value_by_exponent(8, 6);

        // Then
        #[rustfmt::skip]
            let expected_board = Board::from(vec![
            0, 4, 0, 2,
            2, 32, 4, 0,
            64, 2, 0, 512,
            16, 8, 32, 32,
        ]);
        assert_eq!(expected_board, board);
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
    fn should_count_distinct_tiles() {
        // Given
        #[rustfmt::skip]
        let vec_board = vec![
            0, 2, 0, 2048,
            0, 16, 0, 512,
            0, 0, 8, 4,
            8, 2, 16, 64
        ];
        let board = Board::from(vec_board);

        // When
        let distinct_tiles = board.count_distinct_tiles();
        assert_eq!(7, distinct_tiles);
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
| 0     | 2     | 0     | 32768 |
+-------+-------+-------+-------+
| 0     | 256   | 0     | 512   |
+-------+-------+-------+-------+
| 0     | 0     | 1024  | 4     |
+-------+-------+-------+-------+
| 8     | 2     | 16    | 64    |
+-------+-------+-------+-------+
"#;
        assert_eq!(expected_display, display);
    }
}
