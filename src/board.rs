use crate::error::*;
use crate::GameResult;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::from_utf8;

/// `Board` is the main object of the 2048 game. It represents the state of the 16 squares.
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
/// let board_values = vec![
///     0, 0, 0, 0,
///     0, 0, 0, 0,
///     0, 0, 4, 0,
///     0, 16, 0, 8,
/// ];
///
/// let board = Board::try_from(board_values).unwrap();
/// let board_repr: usize = 2usize.pow(0 + 0) + 2usize.pow(1 + 0) + 2usize.pow(2 + 8) + 2usize.pow(1 + 20);
/// assert_eq!(board_repr, board.state);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    state: u64,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let squares: Vec<usize> = self.clone().into();
        let mut display = String::new();
        display.push_str("\n+-------+-------+-------+-------+\n");
        for (i, square) in squares.into_iter().enumerate() {
            display.push_str(&*format!("| {}\t", square));
            if square < 10 {
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

impl TryFrom<Vec<usize>> for Board {
    type Error = Error;

    fn try_from(squares: Vec<usize>) -> GameResult<Self> {
        if squares.len() != 16 {
            return Err(Error::new(
                ErrorKind::InvalidBoardRepr,
                format!("Board does not contain exactly 16 squares: {:?}", squares),
            ));
        }

        let binary_repr = squares
            .iter()
            .map(|square_value| {
                if *square_value == 0 {
                    return Ok("0000".to_string());
                }
                let exponent = ((*square_value as f64).ln() / (2.0 as f64).ln()) as u32;
                if 2usize.pow(exponent) != *square_value || exponent > 15 {
                    return Err(Error::new(
                        ErrorKind::InvalidSquareValue(*square_value),
                        format!("Invalid square value in board {:?}", squares),
                    ));
                }
                Ok(zero_prefix(format!("{:b}", exponent), 4))
            })
            .collect::<GameResult<String>>()?;
        let state = u64::from_str_radix(&binary_repr, 2).unwrap();
        Ok(Self { state })
    }
}

impl From<Board> for Vec<usize> {
    fn from(board: Board) -> Self {
        let binary_repr = zero_prefix(format!("{:b}", board.state), 64);
        binary_repr
            .as_bytes()
            .chunks(4)
            .into_iter()
            .map(|chunk| {
                let exponent = u32::from_str_radix(from_utf8(chunk).unwrap(), 2).unwrap();
                if exponent == 0 {
                    return 0;
                }
                2usize.pow(exponent)
            })
            .collect()
    }
}

fn zero_prefix(repr: String, target_length: usize) -> String {
    let prefix_length = target_length - repr.len();
    if prefix_length == 0 {
        return repr;
    }
    let prefix = std::iter::repeat("0")
        .take(prefix_length)
        .collect::<String>();
    format!("{}{}", prefix, repr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_vec_to_board() {
        // Given
        let vec_board = vec![0, 2, 0, 0, 32768, 0, 0, 2, 0, 0, 16, 4, 8, 2, 16, 64];

        // When
        let board = Board::try_from(vec_board.clone()).unwrap();

        // Then
        let into_vec_board: Vec<usize> = board.into();
        assert_eq!(vec_board, into_vec_board);
    }

    #[test]
    fn should_use_binary_representation() {
        // Given
        let board_values = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 16, 0, 8];

        // When
        let board = Board::try_from(board_values).unwrap();

        // Then
        let board_repr: usize =
            2usize.pow(0 + 0) + 2usize.pow(1 + 0) + 2usize.pow(2 + 8) + 2usize.pow(1 + 20);
        assert_eq!(board_repr as u64, board.state);
    }

    #[test]
    fn should_allow_only_power_of_2() {
        // Given
        let vec_board = vec![0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 15, 4, 8, 2, 16, 64];

        // When
        let board_res = Board::try_from(vec_board.clone());

        // Then
        assert_eq!(
            Some(ErrorKind::InvalidSquareValue(15)),
            board_res.err().map(|e| e.kind())
        );
    }

    #[test]
    fn should_not_allow_value_above_32768() {
        // Given
        let vec_board = vec![0, 2, 0, 32768, 0, 0, 0, 2, 0, 0, 65536, 4, 8, 2, 16, 64];

        // When
        let board_res = Board::try_from(vec_board.clone());

        // Then
        assert_eq!(
            Some(ErrorKind::InvalidSquareValue(65536)),
            board_res.err().map(|e| e.kind())
        );
    }

    #[test]
    fn should_display_board() {
        // Given
        let vec_board = vec![0, 2, 0, 32768, 0, 256, 0, 512, 0, 0, 1024, 4, 8, 2, 16, 64];
        let board = Board::try_from(vec_board).unwrap();

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
