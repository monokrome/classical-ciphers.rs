use crate::Cipher;

/// Planetary magic square cipher implementation.
///
/// Magic squares encode letters by mapping them to positions within
/// a magic square. The letter's numeric value (A=1, B=2, etc.) is
/// found in the square, and the position (row, column) is output.
///
/// Available planetary squares:
/// - Saturn (3x3): values 1-9
/// - Jupiter (4x4): values 1-16
/// - Mars (5x5): values 1-25
/// - Sun (6x6): values 1-36
/// - Venus (7x7): values 1-49
/// - Mercury (8x8): values 1-64
/// - Moon (9x9): values 1-81
#[derive(Debug, Clone)]
pub struct MagicSquare {
    square: Vec<Vec<u32>>,
    size: usize,
    separator: String,
    coord_separator: String,
}

/// The seven classical planetary magic squares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Planet {
    Saturn,
    Jupiter,
    Mars,
    Sun,
    Venus,
    Mercury,
    Moon,
}

impl Planet {
    /// Returns the size of this planet's magic square.
    pub fn size(self) -> usize {
        match self {
            Planet::Saturn => 3,
            Planet::Jupiter => 4,
            Planet::Mars => 5,
            Planet::Sun => 6,
            Planet::Venus => 7,
            Planet::Mercury => 8,
            Planet::Moon => 9,
        }
    }

    /// Returns the magic constant (sum of each row/column/diagonal).
    pub fn magic_constant(self) -> u32 {
        let n = self.size() as u32;
        n * (n * n + 1) / 2
    }
}

impl MagicSquare {
    /// Creates a magic square cipher for the specified planet.
    pub fn new(planet: Planet) -> Self {
        let square = Self::generate_square(planet);
        let size = planet.size();
        Self {
            square,
            size,
            separator: " ".to_string(),
            coord_separator: ",".to_string(),
        }
    }

    /// Creates a Saturn (3x3) magic square cipher.
    pub fn saturn() -> Self {
        Self::new(Planet::Saturn)
    }

    /// Creates a Jupiter (4x4) magic square cipher.
    pub fn jupiter() -> Self {
        Self::new(Planet::Jupiter)
    }

    /// Creates a Mars (5x5) magic square cipher.
    pub fn mars() -> Self {
        Self::new(Planet::Mars)
    }

    /// Creates a Sun (6x6) magic square cipher.
    pub fn sun() -> Self {
        Self::new(Planet::Sun)
    }

    /// Creates a Venus (7x7) magic square cipher.
    pub fn venus() -> Self {
        Self::new(Planet::Venus)
    }

    /// Creates a Mercury (8x8) magic square cipher.
    pub fn mercury() -> Self {
        Self::new(Planet::Mercury)
    }

    /// Creates a Moon (9x9) magic square cipher.
    pub fn moon() -> Self {
        Self::new(Planet::Moon)
    }

    /// Sets the separator between coordinate pairs.
    pub fn with_separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    /// Sets the separator between row and column within a coordinate.
    pub fn with_coord_separator(mut self, sep: &str) -> Self {
        self.coord_separator = sep.to_string();
        self
    }

    /// Returns the maximum letter value this square can encode (A=1).
    pub fn max_value(&self) -> u32 {
        (self.size * self.size) as u32
    }

    fn generate_square(planet: Planet) -> Vec<Vec<u32>> {
        match planet {
            Planet::Saturn => Self::saturn_square(),
            Planet::Jupiter => Self::jupiter_square(),
            Planet::Mars => Self::mars_square(),
            Planet::Sun => Self::sun_square(),
            Planet::Venus => Self::venus_square(),
            Planet::Mercury => Self::mercury_square(),
            Planet::Moon => Self::moon_square(),
        }
    }

    fn saturn_square() -> Vec<Vec<u32>> {
        vec![vec![2, 7, 6], vec![9, 5, 1], vec![4, 3, 8]]
    }

    fn jupiter_square() -> Vec<Vec<u32>> {
        vec![
            vec![4, 14, 15, 1],
            vec![9, 7, 6, 12],
            vec![5, 11, 10, 8],
            vec![16, 2, 3, 13],
        ]
    }

    fn mars_square() -> Vec<Vec<u32>> {
        vec![
            vec![11, 24, 7, 20, 3],
            vec![4, 12, 25, 8, 16],
            vec![17, 5, 13, 21, 9],
            vec![10, 18, 1, 14, 22],
            vec![23, 6, 19, 2, 15],
        ]
    }

    fn sun_square() -> Vec<Vec<u32>> {
        vec![
            vec![6, 32, 3, 34, 35, 1],
            vec![7, 11, 27, 28, 8, 30],
            vec![19, 14, 16, 15, 23, 24],
            vec![18, 20, 22, 21, 17, 13],
            vec![25, 29, 10, 9, 26, 12],
            vec![36, 5, 33, 4, 2, 31],
        ]
    }

    fn venus_square() -> Vec<Vec<u32>> {
        vec![
            vec![22, 47, 16, 41, 10, 35, 4],
            vec![5, 23, 48, 17, 42, 11, 29],
            vec![30, 6, 24, 49, 18, 36, 12],
            vec![13, 31, 7, 25, 43, 19, 37],
            vec![38, 14, 32, 1, 26, 44, 20],
            vec![21, 39, 8, 33, 2, 27, 45],
            vec![46, 15, 40, 9, 34, 3, 28],
        ]
    }

    fn mercury_square() -> Vec<Vec<u32>> {
        vec![
            vec![8, 58, 59, 5, 4, 62, 63, 1],
            vec![49, 15, 14, 52, 53, 11, 10, 56],
            vec![41, 23, 22, 44, 45, 19, 18, 48],
            vec![32, 34, 35, 29, 28, 38, 39, 25],
            vec![40, 26, 27, 37, 36, 30, 31, 33],
            vec![17, 47, 46, 20, 21, 43, 42, 24],
            vec![9, 55, 54, 12, 13, 51, 50, 16],
            vec![64, 2, 3, 61, 60, 6, 7, 57],
        ]
    }

    fn moon_square() -> Vec<Vec<u32>> {
        vec![
            vec![37, 78, 29, 70, 21, 62, 13, 54, 5],
            vec![6, 38, 79, 30, 71, 22, 63, 14, 46],
            vec![47, 7, 39, 80, 31, 72, 23, 55, 15],
            vec![16, 48, 8, 40, 81, 32, 64, 24, 56],
            vec![57, 17, 49, 9, 41, 73, 33, 65, 25],
            vec![26, 58, 18, 50, 1, 42, 74, 34, 66],
            vec![67, 27, 59, 10, 51, 2, 43, 75, 35],
            vec![36, 68, 19, 60, 11, 52, 3, 44, 76],
            vec![77, 28, 69, 20, 61, 12, 53, 4, 45],
        ]
    }

    fn find_position(&self, value: u32) -> Option<(usize, usize)> {
        for (row, row_data) in self.square.iter().enumerate() {
            for (col, &cell) in row_data.iter().enumerate() {
                if cell == value {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn letter_to_value(c: char) -> Option<u32> {
        if c.is_ascii_alphabetic() {
            Some((c.to_ascii_uppercase() as u8 - b'A' + 1) as u32)
        } else {
            None
        }
    }

    fn value_to_letter(value: u32) -> Option<char> {
        if (1..=26).contains(&value) {
            Some((b'A' + value as u8 - 1) as char)
        } else {
            None
        }
    }

    fn encode_letter(&self, c: char) -> Option<String> {
        let value = Self::letter_to_value(c)?;
        if value > self.max_value() {
            return None;
        }
        let (row, col) = self.find_position(value)?;
        Some(format!("{}{}{}", row + 1, self.coord_separator, col + 1))
    }
}

impl Cipher for MagicSquare {
    fn encrypt(&self, input: &str) -> String {
        let mut result = Vec::new();

        for c in input.chars() {
            if c.is_ascii_alphabetic() {
                if let Some(encoded) = self.encode_letter(c) {
                    result.push(encoded);
                } else {
                    result.push(c.to_string());
                }
            } else {
                result.push(c.to_string());
            }
        }

        let mut output = String::new();
        for (i, s) in result.iter().enumerate() {
            if i > 0 && s.contains(&self.coord_separator) {
                if let Some(prev) = result.get(i - 1) {
                    if prev.contains(&self.coord_separator) {
                        output.push_str(&self.separator);
                    }
                }
            }
            output.push_str(s);
        }
        output
    }

    fn decrypt(&self, input: &str) -> String {
        let mut result = String::new();
        let parts: Vec<&str> = input.split(&self.separator).collect();

        for part in parts {
            if part.contains(&self.coord_separator) {
                let coords: Vec<&str> = part.split(&self.coord_separator).collect();
                if coords.len() == 2 {
                    if let (Ok(row), Ok(col)) =
                        (coords[0].parse::<usize>(), coords[1].parse::<usize>())
                    {
                        if row >= 1 && row <= self.size && col >= 1 && col <= self.size {
                            let value = self.square[row - 1][col - 1];
                            if let Some(letter) = Self::value_to_letter(value) {
                                result.push(letter);
                                continue;
                            }
                        }
                    }
                }
            }
            result.push_str(part);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturn_encrypt_basic() {
        let cipher = MagicSquare::saturn();
        // A=1, which is at position (2,3) in the Saturn square
        assert_eq!(cipher.encrypt("A"), "2,3");
    }

    #[test]
    fn saturn_round_trip() {
        let cipher = MagicSquare::saturn();
        let plaintext = "ABCDEFGHI";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn jupiter_round_trip() {
        let cipher = MagicSquare::jupiter();
        let plaintext = "ABCDEFGHIJKLMNOP";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn mars_round_trip() {
        let cipher = MagicSquare::mars();
        let plaintext = "ABCDEFGHIJKLMNOPQRSTUVWXY";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn sun_round_trip() {
        let cipher = MagicSquare::sun();
        let plaintext = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn venus_round_trip() {
        let cipher = MagicSquare::venus();
        let plaintext = "HELLO";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn mercury_round_trip() {
        let cipher = MagicSquare::mercury();
        let plaintext = "THEQUICKBROWNFOX";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn moon_round_trip() {
        let cipher = MagicSquare::moon();
        let plaintext = "JUMPSOVERTHELAZYDOG";
        let encrypted = cipher.encrypt(plaintext);
        assert_eq!(cipher.decrypt(&encrypted), plaintext);
    }

    #[test]
    fn letter_out_of_range() {
        let cipher = MagicSquare::saturn();
        let result = cipher.encrypt("Z");
        assert_eq!(result, "Z");
    }

    #[test]
    fn preserves_non_alpha() {
        let cipher = MagicSquare::mars();
        let result = cipher.encrypt("A, B!");
        assert!(result.contains(','));
        assert!(result.contains('!'));
    }

    #[test]
    fn custom_separator() {
        let cipher = MagicSquare::saturn().with_separator("-");
        let encrypted = cipher.encrypt("AB");
        assert!(encrypted.contains('-'));
        assert_eq!(cipher.decrypt(&encrypted), "AB");
    }

    #[test]
    fn custom_coord_separator() {
        let cipher = MagicSquare::saturn().with_coord_separator(".");
        let encrypted = cipher.encrypt("A");
        assert!(encrypted.contains('.'));
        assert_eq!(cipher.decrypt(&encrypted), "A");
    }

    #[test]
    fn planet_sizes() {
        assert_eq!(Planet::Saturn.size(), 3);
        assert_eq!(Planet::Jupiter.size(), 4);
        assert_eq!(Planet::Mars.size(), 5);
        assert_eq!(Planet::Sun.size(), 6);
        assert_eq!(Planet::Venus.size(), 7);
        assert_eq!(Planet::Mercury.size(), 8);
        assert_eq!(Planet::Moon.size(), 9);
    }

    #[test]
    fn magic_constants() {
        assert_eq!(Planet::Saturn.magic_constant(), 15);
        assert_eq!(Planet::Jupiter.magic_constant(), 34);
        assert_eq!(Planet::Mars.magic_constant(), 65);
        assert_eq!(Planet::Sun.magic_constant(), 111);
        assert_eq!(Planet::Venus.magic_constant(), 175);
        assert_eq!(Planet::Mercury.magic_constant(), 260);
        assert_eq!(Planet::Moon.magic_constant(), 369);
    }

    #[test]
    fn saturn_is_valid_magic_square() {
        let cipher = MagicSquare::saturn();
        let expected = Planet::Saturn.magic_constant();

        for row in &cipher.square {
            let sum: u32 = row.iter().sum();
            assert_eq!(sum, expected);
        }

        for col in 0..3 {
            let sum: u32 = (0..3).map(|row| cipher.square[row][col]).sum();
            assert_eq!(sum, expected);
        }
    }

    #[test]
    fn case_insensitive() {
        let cipher = MagicSquare::mars();
        assert_eq!(cipher.encrypt("a"), cipher.encrypt("A"));
    }

    #[test]
    fn max_value() {
        assert_eq!(MagicSquare::saturn().max_value(), 9);
        assert_eq!(MagicSquare::jupiter().max_value(), 16);
        assert_eq!(MagicSquare::mars().max_value(), 25);
        assert_eq!(MagicSquare::moon().max_value(), 81);
    }
}
