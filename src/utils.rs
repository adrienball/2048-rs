pub fn get_exponent(value: u16) -> u64 {
    if value == 0 {
        return 0;
    }
    let mut exponent = 1;
    let mut v = value >> 2;
    while v != 0 {
        v = v >> 1;
        exponent += 1;
    }
    exponent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_exponent() {
        assert_eq!(0, get_exponent(0));
        assert_eq!(3, get_exponent(8));
        assert_eq!(5, get_exponent(32));
    }
}
