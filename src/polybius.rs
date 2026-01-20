use crate::Cipher;

/// Polybius square cipher implementation.
///
/// Uses a 5x5 grid where I/J are combined. Each letter is encoded as
/// a two-digit coordinate (row, column) using digits 1-5.
///
/// Standard grid:
/// ```text
///   1 2 3 4 5
/// 1 A B C D E
/// 2 F G H I K
/// 3 L M N O P
/// 4 Q R S T U
/// 5 V W X Y Z
/// ```
#[derive(Debug, Clone)]
pub struct Polybius {
    grid: [[char; 5]; 5],
    separator: String,
}

impl Default for Polybius {
    fn default() -> Self {
        Self::new()
    }
}

impl Polybius {
    /// Creates a new Polybius square with the standard alphabet.
    pub fn new() -> Self {
        Self::with_alphabet("ABCDEFGHIKLMNOPQRSTUVWXYZ")
    }

    /// Creates a Polybius square with a custom 25-character alphabet.
    ///
    /// The alphabet must contain exactly 25 unique uppercase characters.
    /// If invalid, falls back to the standard alphabet.
    pub fn with_alphabet(alphabet: &str) -> Self {
        let chars: Vec<char> = alphabet.chars().collect();
        let grid = if chars.len() == 25 {
            let mut grid = [[' '; 5]; 5];
            for (i, &c) in chars.iter().enumerate() {
                grid[i / 5][i % 5] = c;
            }
            grid
        } else {
            Self::standard_grid()
        };

        Self {
            grid,
            separator: String::new(),
        }
    }

    /// Creates a Polybius square with a keyed alphabet.
    ///
    /// The key is prepended to the alphabet, with duplicate letters removed.
    pub fn with_key(key: &str) -> Self {
        let mut seen = [false; 26];
        let mut alphabet = String::with_capacity(25);

        for c in key.to_uppercase().chars().chain('A'..='Z') {
            if !c.is_ascii_alphabetic() {
                continue;
            }
            let normalized = if c == 'J' { 'I' } else { c };
            let idx = (normalized as u8 - b'A') as usize;
            if !seen[idx] {
                seen[idx] = true;
                alphabet.push(normalized);
            }
        }

        Self::with_alphabet(&alphabet)
    }

    /// Sets the separator between coordinate pairs in the output.
    pub fn with_separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    fn standard_grid() -> [[char; 5]; 5] {
        [
            ['A', 'B', 'C', 'D', 'E'],
            ['F', 'G', 'H', 'I', 'K'],
            ['L', 'M', 'N', 'O', 'P'],
            ['Q', 'R', 'S', 'T', 'U'],
            ['V', 'W', 'X', 'Y', 'Z'],
        ]
    }

    fn find_position(&self, c: char) -> Option<(usize, usize)> {
        let upper = c.to_ascii_uppercase();
        let target = if upper == 'J' { 'I' } else { upper };

        for (row, grid_row) in self.grid.iter().enumerate() {
            for (col, &cell) in grid_row.iter().enumerate() {
                if cell == target {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn encode_char(&self, c: char) -> Option<String> {
        self.find_position(c)
            .map(|(row, col)| format!("{}{}", row + 1, col + 1))
    }
}

impl Cipher for Polybius {
    fn encrypt(&self, input: &str) -> String {
        let encoded: Vec<String> = input
            .chars()
            .filter_map(|c| {
                if c.is_ascii_alphabetic() {
                    self.encode_char(c)
                } else {
                    Some(c.to_string())
                }
            })
            .collect();

        if self.separator.is_empty() {
            encoded.join("")
        } else {
            let mut result = String::new();
            for (i, s) in encoded.iter().enumerate() {
                if i > 0 && s.len() == 2 && encoded.get(i - 1).is_some_and(|p| p.len() == 2) {
                    result.push_str(&self.separator);
                }
                result.push_str(s);
            }
            result
        }
    }

    fn decrypt(&self, input: &str) -> String {
        let cleaned: String = input
            .chars()
            .filter(|c| c.is_ascii_digit() || !c.is_ascii_alphanumeric() && *c != ' ')
            .collect();

        let digits: Vec<u8> = cleaned
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        let mut result = String::new();
        let mut input_chars = input.chars().peekable();
        let mut digit_idx = 0;

        while input_chars.peek().is_some() {
            let c = input_chars.next().unwrap();

            if c.is_ascii_digit() {
                if digit_idx + 1 < digits.len() {
                    let row = (digits[digit_idx] - 1) as usize;
                    let col = (digits[digit_idx + 1] - 1) as usize;

                    if row < 5 && col < 5 {
                        result.push(self.grid[row][col]);
                    }
                    digit_idx += 2;

                    if let Some(&next) = input_chars.peek() {
                        if next.is_ascii_digit() {
                            continue;
                        }
                    }
                }
            } else if !self.separator.is_empty() && self.separator.starts_with(c) {
                let sep_len = self.separator.len();
                let mut matched = String::from(c);
                while matched.len() < sep_len {
                    if let Some(&next) = input_chars.peek() {
                        if self.separator[matched.len()..].starts_with(next) {
                            matched.push(input_chars.next().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if matched != self.separator {
                    result.push_str(&matched);
                }
            } else {
                result.push(c);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_basic() {
        let cipher = Polybius::new();
        assert_eq!(cipher.encrypt("HELLO"), "2315313134");
    }

    #[test]
    fn decrypt_basic() {
        let cipher = Polybius::new();
        assert_eq!(cipher.decrypt("2315313134"), "HELLO");
    }

    #[test]
    fn round_trip() {
        let cipher = Polybius::new();
        let plaintext = "THEQUICKBROWNFOX";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn handles_j_as_i() {
        let cipher = Polybius::new();
        assert_eq!(cipher.encrypt("J"), "24");
        assert_eq!(cipher.encrypt("I"), "24");
    }

    #[test]
    fn preserves_non_alpha() {
        let cipher = Polybius::new();
        let result = cipher.encrypt("A,B.C");
        assert_eq!(result, "11,12.13");
    }

    #[test]
    fn with_separator() {
        let cipher = Polybius::new().with_separator(" ");
        assert_eq!(cipher.encrypt("ABC"), "11 12 13");
    }

    #[test]
    fn decrypt_with_separator() {
        let cipher = Polybius::new().with_separator(" ");
        assert_eq!(cipher.decrypt("11 12 13"), "ABC");
    }

    #[test]
    fn with_key() {
        let cipher = Polybius::with_key("KEYWORD");
        let encrypted = cipher.encrypt("HELLO");
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, "HELLO");
    }

    #[test]
    fn case_insensitive_input() {
        let cipher = Polybius::new();
        assert_eq!(cipher.encrypt("hello"), cipher.encrypt("HELLO"));
    }

    #[test]
    fn default_impl() {
        let cipher = Polybius::default();
        assert_eq!(cipher.encrypt("A"), "11");
    }

    #[test]
    fn keyed_alphabet_removes_duplicates() {
        let cipher = Polybius::with_key("AAABBBCCC");
        let encrypted = cipher.encrypt("ABC");
        assert_ne!(encrypted, "111111");
    }

    #[test]
    fn full_alphabet_round_trip() {
        let cipher = Polybius::new();
        let plaintext = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
        let encrypted = cipher.encrypt(plaintext);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, plaintext);
    }
}
