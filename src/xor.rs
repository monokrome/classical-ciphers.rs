use crate::Cipher;

/// XOR cipher - symmetric encryption using repeating key
#[derive(Debug, Clone)]
pub struct Xor {
    key: Vec<u8>,
}

impl Xor {
    pub fn new(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }

    pub fn with_str_key(key: &str) -> Self {
        Self::new(key.as_bytes())
    }

    fn transform(&self, input: &str) -> String {
        if self.key.is_empty() {
            return input.to_string();
        }

        input
            .bytes()
            .enumerate()
            .map(|(i, b)| {
                let key_byte = self.key[i % self.key.len()];
                (b ^ key_byte) as char
            })
            .collect()
    }

    /// XOR raw bytes, returning bytes (useful for binary data)
    pub fn transform_bytes(&self, input: &[u8]) -> Vec<u8> {
        if self.key.is_empty() {
            return input.to_vec();
        }

        input
            .iter()
            .enumerate()
            .map(|(i, &b)| {
                let key_byte = self.key[i % self.key.len()];
                b ^ key_byte
            })
            .collect()
    }
}

impl Cipher for Xor {
    fn encrypt(&self, input: &str) -> String {
        self.transform(input)
    }

    fn decrypt(&self, input: &str) -> String {
        self.transform(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symmetric() {
        let cipher = Xor::with_str_key("KEY");
        let original = "Hello";
        let encrypted = cipher.encrypt(original);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, original);
    }

    #[test]
    fn single_byte_key() {
        let cipher = Xor::new(&[0x20]);
        assert_eq!(cipher.encrypt("HELLO"), "hello");
        assert_eq!(cipher.encrypt("hello"), "HELLO");
    }

    #[test]
    fn empty_key_passthrough() {
        let cipher = Xor::new(&[]);
        assert_eq!(cipher.encrypt("Hello"), "Hello");
    }

    #[test]
    fn transform_bytes() {
        let cipher = Xor::new(&[0xFF]);
        let input = vec![0x00, 0x0F, 0xF0];
        let output = cipher.transform_bytes(&input);
        assert_eq!(output, vec![0xFF, 0xF0, 0x0F]);
    }
}
