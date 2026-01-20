# classical-ciphers

Simple implementations of classical ciphers in Rust.

## Ciphers

- **Caesar** - Shift cipher with configurable offset (includes ROT13)
- **Vigenère** - Polyalphabetic substitution using a keyword
- **Atbash** - Alphabet reversal (A↔Z, B↔Y, etc.)
- **XOR** - Symmetric XOR with repeating key

## Usage

```rust
use classical_ciphers::{Caesar, Vigenere, Atbash, Xor, Cipher};

// Caesar cipher
let caesar = Caesar::new(3);
assert_eq!(caesar.encrypt("HELLO"), "KHOOR");
assert_eq!(caesar.decrypt("KHOOR"), "HELLO");

// ROT13
let rot13 = Caesar::rot13();
assert_eq!(rot13.encrypt("HELLO"), "URYYB");

// Vigenère cipher
let vigenere = Vigenere::new("KEY");
assert_eq!(vigenere.encrypt("HELLO"), "RIJVS");

// Atbash cipher
let atbash = Atbash::new();
assert_eq!(atbash.encrypt("ABC"), "ZYX");

// XOR cipher
let xor = Xor::with_str_key("KEY");
let encrypted = xor.encrypt("Hello");
let decrypted = xor.decrypt(&encrypted);
assert_eq!(decrypted, "Hello");
```

## License

MIT
