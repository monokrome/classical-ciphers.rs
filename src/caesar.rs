use crate::Cipher;

/// Caesar cipher - shifts each letter by a fixed amount
#[derive(Debug, Clone, Copy)]
pub struct Caesar {
    shift: i32,
}

impl Caesar {
    pub fn new(shift: i32) -> Self {
        Self { shift }
    }

    /// ROT13 is Caesar with shift 13
    pub fn rot13() -> Self {
        Self::new(13)
    }

    fn shift_char(&self, c: char, shift: i32) -> char {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = (c as u8 - base) as i32;
            let shifted = ((offset + shift).rem_euclid(26)) as u8;
            (base + shifted) as char
        } else {
            c
        }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.shift_char(c, self.shift))
            .collect()
    }

    fn decrypt(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.shift_char(c, -self.shift))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_shift_3() {
        let cipher = Caesar::new(3);
        assert_eq!(cipher.encrypt("ABC"), "DEF");
        assert_eq!(cipher.encrypt("XYZ"), "ABC");
        assert_eq!(cipher.encrypt("Hello"), "Khoor");
    }

    #[test]
    fn decrypt_shift_3() {
        let cipher = Caesar::new(3);
        assert_eq!(cipher.decrypt("DEF"), "ABC");
        assert_eq!(cipher.decrypt("ABC"), "XYZ");
        assert_eq!(cipher.decrypt("Khoor"), "Hello");
    }

    #[test]
    fn roundtrip() {
        let cipher = Caesar::new(13);
        let original = "The Quick Brown Fox";
        let encrypted = cipher.encrypt(original);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, original);
    }

    #[test]
    fn rot13() {
        let cipher = Caesar::rot13();
        assert_eq!(cipher.encrypt("Hello"), "Uryyb");
        assert_eq!(cipher.encrypt("Uryyb"), "Hello");
    }

    #[test]
    fn preserves_non_alpha() {
        let cipher = Caesar::new(5);
        assert_eq!(cipher.encrypt("Hello, World! 123"), "Mjqqt, Btwqi! 123");
    }

    #[test]
    fn negative_shift() {
        let cipher = Caesar::new(-3);
        assert_eq!(cipher.encrypt("DEF"), "ABC");
    }
}
