pub fn get_exponent(value: u16) -> u64 {
    match value {
        0 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        32 => 5,
        64 => 6,
        128 => 7,
        256 => 8,
        512 => 9,
        1024 => 10,
        2048 => 11,
        4096 => 12,
        8192 => 13,
        16384 => 14,
        32768 => 15,
        _ => panic!("Invalid tile value {}", value),
    }
}

pub fn build_left_moves_table() -> Vec<u16> {
    let mut table: Vec<u16> = (0..(std::u16::MAX as usize + 1))
        .into_iter()
        .map(|x| x as u16)
        .collect();
    for i1 in 0..16 {
        for i2 in 0..16 {
            for i3 in 0..16 {
                for i4 in 0..16 {
                    let row: u16 = (i1 << 12) | (i2 << 8) | (i3 << 4) | i4;
                    let left_move = get_left_move(row);
                    table[row as usize] = left_move;
                }
            }
        }
    }
    table
}

pub fn build_right_moves_table() -> Vec<u16> {
    let mut table: Vec<u16> = (0..(std::u16::MAX as usize + 1))
        .into_iter()
        .map(|x| x as u16)
        .collect();
    for i1 in 0..16 {
        for i2 in 0..16 {
            for i3 in 0..16 {
                for i4 in 0..16 {
                    let row: u16 = (i1 << 12) | (i2 << 8) | (i3 << 4) | i4;
                    let right_move = get_right_move(row);
                    table[row as usize] = right_move;
                }
            }
        }
    }
    table
}

fn get_left_move(row: u16) -> u16 {
    let mut result = row;
    let mut value_idx = 0;
    let mut prev_value = std::u8::MAX;
    let mut new_value_idx = value_idx;
    // whether or not tiles have been moved in this row
    let mut moved = false;
    for i in 0..4 {
        let value: u8 = ((row & (0b1111 << 4 * (3 - i))) >> 4 * (3 - i)) as u8;
        if value == 0 {
            moved = true;
        } else if value == prev_value {
            result = set_value_in_row(result, new_value_idx - 1, value + 1);
            result = set_value_in_row(result, value_idx, 0);
            prev_value = std::u8::MAX;
            moved = true;
        } else {
            if moved {
                result = set_value_in_row(result, new_value_idx, value);
                result = set_value_in_row(result, value_idx, 0);
            }
            prev_value = value;
            new_value_idx += 1;
        }
        value_idx += 1;
    }
    result
}

fn get_right_move(row: u16) -> u16 {
    let mut result = row;
    let mut value_idx = 3;
    let mut prev_value = std::u8::MAX;
    let mut new_value_idx = value_idx;
    // whether or not cells have been moved in this row
    let mut moved = false;
    for i in (0..4).rev() {
        let value: u8 = ((row & (0b1111 << 4 * (3 - i))) >> 4 * (3 - i)) as u8;
        if value == 0 {
            moved = true;
        } else if value == prev_value {
            result = set_value_in_row(result, new_value_idx + 1, value + 1);
            result = set_value_in_row(result, value_idx, 0);
            prev_value = std::u8::MAX;
            moved = true;
        } else {
            if moved {
                result = set_value_in_row(result, new_value_idx, value);
                result = set_value_in_row(result, value_idx, 0);
            }
            prev_value = value;
            if new_value_idx > 0 {
                new_value_idx -= 1;
            }
        }
        if value_idx > 0 {
            value_idx -= 1;
        }
    }
    result
}

fn set_value_in_row(row: u16, idx: u8, value: u8) -> u16 {
    // bitmask with 0000 at the corresponding tile_idx and 1s everywhere else
    let clear_mask: u16 = !(0b1111u16 << (4 * (3 - idx) as u16));
    let update_mask: u16 = (value as u16) << (4 * (3 - idx) as u16);
    (row & clear_mask) | update_mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_value_in_row() {
        // Given
        let row = 0b0101000001011100;

        // When
        let updated_row = set_value_in_row(row, 2, 8);

        // Then
        assert_eq!(0b0101000010001100, updated_row);
    }

    #[test]
    fn should_get_left_move() {
        // Given
        let row = 0b0101000001011100;

        // When
        let left_moved = get_left_move(row);

        // Then
        assert_eq!(0b0110110000000000, left_moved);
    }

    #[test]
    fn should_get_right_move() {
        // Given
        let row = 0b0101000001011100;

        // When
        let left_moved = get_right_move(row);

        // Then
        assert_eq!(0b0000000001101100, left_moved);
    }
}
