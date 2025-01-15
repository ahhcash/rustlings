// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        let thirtytwo = power_of_2(5);
        let eight = power_of_2(3);
        let one = power_of_2(0);
        let sixteen = power_of_2(4);
        assert_eq!(sixteen, 16);
        assert_eq!(thirtytwo, 32);
        assert_eq!(one, 1);
        assert_eq!(eight, 8);
    }
}
