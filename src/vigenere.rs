use crate::Cipher;

/// Vigenère cipher - polyalphabetic substitution using a keyword
#[derive(Debug, Clone)]
pub struct Vigenere {
    key: Vec<u8>,
}

impl Vigenere {
    pub fn new(key: &str) -> Self {
        let key: Vec<u8> = key
            .to_uppercase()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| (c as u8) - b'A')
            .collect();
        Self { key }
    }

    fn transform(&self, input: &str, decrypt: bool) -> String {
        if self.key.is_empty() {
            return input.to_string();
        }

        let mut key_index = 0;
        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    let offset = (c as u8 - base) as i32;
                    let key_shift = self.key[key_index % self.key.len()] as i32;
                    key_index += 1;

                    let shifted = if decrypt {
                        (offset - key_shift).rem_euclid(26)
                    } else {
                        (offset + key_shift).rem_euclid(26)
                    };
                    (base + shifted as u8) as char
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Cipher for Vigenere {
    fn encrypt(&self, input: &str) -> String {
        self.transform(input, false)
    }

    fn decrypt(&self, input: &str) -> String {
        self.transform(input, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_basic() {
        let cipher = Vigenere::new("KEY");
        assert_eq!(cipher.encrypt("HELLO"), "RIJVS");
    }

    #[test]
    fn decrypt_basic() {
        let cipher = Vigenere::new("KEY");
        assert_eq!(cipher.decrypt("RIJVS"), "HELLO");
    }

    #[test]
    fn roundtrip() {
        let cipher = Vigenere::new("SECRET");
        let original = "The Quick Brown Fox";
        let encrypted = cipher.encrypt(original);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, original);
    }

    #[test]
    fn preserves_case() {
        let cipher = Vigenere::new("ABC");
        let result = cipher.encrypt("Hello World");
        assert_eq!(result, "Hfnlp Yosnd");
    }

    #[test]
    fn empty_key_passthrough() {
        let cipher = Vigenere::new("");
        assert_eq!(cipher.encrypt("Hello"), "Hello");
    }
}
