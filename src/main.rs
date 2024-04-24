fn main() {
    println!("Hello, world!");
}

mod gates {
    fn validate_input(a: u8, b: u8) {
        match (a, b) {
            (0, 0) | (0, 1) | (1, 0) | (1, 1) => (),
            _ => panic!("Inputs must be either 0 or 1"),
        }
    }

    pub fn and(a: u8, b: u8) -> u8 {
        validate_input(a, b);
        a & b
    }

    pub fn or(a: u8, b: u8) -> u8 {
        validate_input(a, b);
        a | b
    }

    pub fn not(a: u8) -> u8 {
        match a {
            0 | 1 => a ^ 1,
            _ => panic!("Input must be either 0 or 1"),
        }
    }

    pub fn nand(a: u8, b: u8) -> u8 {
        validate_input(a, b);
        (a & b) ^ 1
    }

    pub fn xor(a: u8, b: u8) -> u8 {
        validate_input(a, b);
        a ^ b
    }
}

mod chips {
    use crate::gates;

    pub fn half_adder(a: u8, b: u8) -> (u8, u8) {
        let sum = gates::xor(a, b);
        let carry = gates::and(a, b);
        (sum, carry)
    }

    pub fn full_adder(a: u8, b: u8, c: u8) -> (u8, u8) {
        let (sum1, carry1) = half_adder(a, b);
        let (sum2, carry2) = half_adder(sum1, c);
        let carry_out = gates::or(carry1, carry2);
        (sum2, carry_out)
    }

    pub fn adder_16(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
        let mut result = [0; 16];
        let mut carry = 0;

        for i in 0..16 {
            let (sum, new_carry) = full_adder(a[i], b[i], carry);
            result[i] = sum;
            carry = new_carry;
        }

        result
    }

    pub fn inc_16(a: [u8; 16]) -> [u8; 16] {
        let one = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        adder_16(a, one)
    }

    pub fn mux_16(a: [u8; 16], b: [u8; 16], selector: u8) -> [u8; 16] {
        match selector {
            0 => a,
            1 => b,
            _ => panic!("invalid selector"),
        }
    }

    pub fn not_16(a: [u8; 16]) -> [u8; 16] {
        let mut result = [0; 16];
        for i in 0..16 {
            result[i] = gates::not(a[i]);
        }
        result
    }

    pub fn and_16(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
        let mut result = [0; 16];
        for i in 0..16 {
            result[i] = gates::and(a[i], b[i]);
        }
        result
    }
}

mod alu {

    use crate::chips;

    pub fn alu(
        x: [u8; 16],
        y: [u8; 16],
        zx: u8,
        nx: u8,
        zy: u8,
        ny: u8,
        f: u8,
        no: u8,
    ) -> ([u8; 16], u8, u8) {
        let x = chips::mux_16(x, [0; 16], zx);
        let y = chips::mux_16(y, [0; 16], zy);

        let x = chips::mux_16(x, chips::not_16(x), nx);
        let y = chips::mux_16(y, chips::not_16(y), ny);

        let result = match f {
            0 => chips::and_16(x, y),
            1 => chips::adder_16(x, y),
            _ => panic!("invalid function code"),
        };

        (chips::mux_16(result, chips::not_16(result), no), 0, 0) // TODO:  zr, ng
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(gates::and(0, 0), 0);
        assert_eq!(gates::and(0, 1), 0);
        assert_eq!(gates::and(1, 0), 0);
        assert_eq!(gates::and(1, 1), 1);
    }

    #[test]
    fn test_or() {
        assert_eq!(gates::or(0, 0), 0);
        assert_eq!(gates::or(0, 1), 1);
        assert_eq!(gates::or(1, 0), 1);
        assert_eq!(gates::or(1, 1), 1);
    }

    #[test]
    fn test_not() {
        assert_eq!(gates::not(1), 0);
        assert_eq!(gates::not(0), 1);
    }

    #[test]
    fn test_nand() {
        assert_eq!(gates::nand(0, 0), 1);
        assert_eq!(gates::nand(0, 1), 1);
        assert_eq!(gates::nand(1, 0), 1);
        assert_eq!(gates::nand(1, 1), 0);
    }

    #[test]
    fn test_xor() {
        assert_eq!(gates::xor(0, 0), 0);
        assert_eq!(gates::xor(0, 1), 1);
        assert_eq!(gates::xor(1, 0), 1);
        assert_eq!(gates::xor(1, 1), 0);
    }

    #[test]
    fn test_half_adder() {
        assert_eq!(chips::half_adder(0, 0), (0, 0));
        assert_eq!(chips::half_adder(0, 1), (1, 0));
        assert_eq!(chips::half_adder(1, 0), (1, 0));
        assert_eq!(chips::half_adder(1, 1), (0, 1));
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(chips::full_adder(0, 0, 0), (0, 0));
        assert_eq!(chips::full_adder(0, 0, 1), (1, 0));
        assert_eq!(chips::full_adder(0, 1, 0), (1, 0));
        assert_eq!(chips::full_adder(0, 1, 1), (0, 1));
        assert_eq!(chips::full_adder(1, 0, 0), (1, 0));
        assert_eq!(chips::full_adder(1, 0, 1), (0, 1));
        assert_eq!(chips::full_adder(1, 1, 0), (0, 1));
        assert_eq!(chips::full_adder(1, 1, 1), (1, 1));
    }

    #[test]
    fn test_adder_16() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0]; // 10922 in binary
        let b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1]; // 5461 in binary

        let sum = chips::adder_16(a, b);

        assert_eq!(sum, [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_inc_16() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let sum = chips::inc_16(a);

        assert_eq!(sum, [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1]);
    }

    #[test]
    fn test_alu_addition() {
        let x = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0]; // 10922 in binary
        let y = [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1]; // 5461 in binary

        let result = alu::alu(x, y, 0, 0, 0, 0, 1, 0); // Perform addition
        assert_eq!(result.0, [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]);
    }
    #[test]
    fn test_alu_negation() {
        let x = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0]; // 10922 in binary
        let y = [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1]; // 5461 in binary

        let result = alu::alu(x, y, 0, 0, 0, 0, 0, 0); // Perform negatation
        assert_eq!(result.0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
