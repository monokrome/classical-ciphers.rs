use crate::Cipher;

/// Atbash cipher - reverses the alphabet (A↔Z, B↔Y, etc.)
#[derive(Debug, Clone, Copy, Default)]
pub struct Atbash;

impl Atbash {
    pub fn new() -> Self {
        Self
    }

    fn transform_char(c: char) -> char {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = c as u8 - base;
            (base + (25 - offset)) as char
        } else {
            c
        }
    }
}

impl Cipher for Atbash {
    fn encrypt(&self, input: &str) -> String {
        input.chars().map(Self::transform_char).collect()
    }

    fn decrypt(&self, input: &str) -> String {
        self.encrypt(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_transform() {
        let cipher = Atbash::new();
        assert_eq!(cipher.encrypt("ABC"), "ZYX");
        assert_eq!(cipher.encrypt("XYZ"), "CBA");
    }

    #[test]
    fn symmetric() {
        let cipher = Atbash::new();
        let original = "Hello World";
        let encrypted = cipher.encrypt(original);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, original);
    }

    #[test]
    fn preserves_case() {
        let cipher = Atbash::new();
        assert_eq!(cipher.encrypt("AbCdEf"), "ZyXwVu");
    }

    #[test]
    fn preserves_non_alpha() {
        let cipher = Atbash::new();
        assert_eq!(cipher.encrypt("Hello, World! 123"), "Svool, Dliow! 123");
    }
}
