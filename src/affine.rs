use crate::Cipher;

/// Affine cipher implementation.
///
/// The affine cipher is a monoalphabetic substitution cipher that uses
/// the formula E(x) = (ax + b) mod 26 for encryption and
/// D(x) = a⁻¹(x - b) mod 26 for decryption.
///
/// The key consists of two values:
/// - `a`: must be coprime with 26 (valid values: 1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25)
/// - `b`: any value from 0-25
#[derive(Debug, Clone, Copy)]
pub struct Affine {
    a: i32,
    a_inv: i32,
    b: i32,
}

impl Affine {
    /// Creates a new Affine cipher with the given keys.
    ///
    /// Returns `None` if `a` is not coprime with 26.
    pub fn new(a: i32, b: i32) -> Option<Self> {
        let a_inv = mod_inverse(a, 26)?;
        Some(Self {
            a: a.rem_euclid(26),
            a_inv,
            b: b.rem_euclid(26),
        })
    }

    /// Creates a Caesar cipher (special case where a=1).
    pub fn caesar(shift: i32) -> Self {
        Self {
            a: 1,
            a_inv: 1,
            b: shift.rem_euclid(26),
        }
    }

    /// Creates a ROT13 cipher (special case where a=1, b=13).
    pub fn rot13() -> Self {
        Self::caesar(13)
    }

    fn transform_char(&self, c: char, encrypt: bool) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }

        let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
        let x = (c as u8 - base) as i32;

        let result = if encrypt {
            (self.a * x + self.b).rem_euclid(26)
        } else {
            (self.a_inv * (x - self.b)).rem_euclid(26)
        };

        (base + result as u8) as char
    }
}

impl Cipher for Affine {
    fn encrypt(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.transform_char(c, true))
            .collect()
    }

    fn decrypt(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.transform_char(c, false))
            .collect()
    }
}

/// Computes the modular multiplicative inverse of `a` modulo `m`.
/// Returns `None` if no inverse exists (i.e., gcd(a, m) != 1).
fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    let a = a.rem_euclid(m);
    let (mut old_r, mut r) = (a, m);
    let (mut old_s, mut s) = (1i32, 0i32);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    if old_r != 1 {
        return None;
    }

    Some(old_s.rem_euclid(m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_basic() {
        let cipher = Affine::new(5, 8).unwrap();
        assert_eq!(cipher.encrypt("HELLO"), "RCLLA");
    }

    #[test]
    fn decrypt_basic() {
        let cipher = Affine::new(5, 8).unwrap();
        assert_eq!(cipher.decrypt("RCLLA"), "HELLO");
    }

    #[test]
    fn round_trip() {
        let cipher = Affine::new(7, 3).unwrap();
        let plaintext = "The Quick Brown Fox";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn preserves_case() {
        let cipher = Affine::new(5, 8).unwrap();
        let result = cipher.encrypt("HeLLo");
        assert_eq!(result, "RcLLa");
    }

    #[test]
    fn preserves_non_alpha() {
        let cipher = Affine::new(5, 8).unwrap();
        assert_eq!(cipher.encrypt("Hello, World! 123"), "Rclla, Oaplx! 123");
    }

    #[test]
    fn invalid_key_a() {
        assert!(Affine::new(2, 5).is_none());
        assert!(Affine::new(4, 5).is_none());
        assert!(Affine::new(6, 5).is_none());
        assert!(Affine::new(8, 5).is_none());
        assert!(Affine::new(10, 5).is_none());
        assert!(Affine::new(12, 5).is_none());
        assert!(Affine::new(13, 5).is_none());
        assert!(Affine::new(14, 5).is_none());
    }

    #[test]
    fn valid_keys() {
        for a in [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25] {
            assert!(Affine::new(a, 0).is_some(), "a={} should be valid", a);
        }
    }

    #[test]
    fn caesar_special_case() {
        let affine = Affine::caesar(3);
        assert_eq!(affine.encrypt("ABC"), "DEF");
        assert_eq!(affine.decrypt("DEF"), "ABC");
    }

    #[test]
    fn rot13_special_case() {
        let cipher = Affine::rot13();
        assert_eq!(cipher.encrypt("HELLO"), "URYYB");
        assert_eq!(cipher.encrypt("URYYB"), "HELLO");
    }

    #[test]
    fn negative_keys() {
        let cipher = Affine::new(-1, -1).unwrap();
        let plaintext = "TEST";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn mod_inverse_correctness() {
        assert_eq!(mod_inverse(5, 26), Some(21));
        assert_eq!(mod_inverse(7, 26), Some(15));
        assert_eq!(mod_inverse(2, 26), None);
    }
}
