mod affine;
mod atbash;
mod caesar;
mod magic_square;
mod polybius;
mod vigenere;
mod xor;

pub use affine::Affine;
pub use atbash::Atbash;
pub use caesar::Caesar;
pub use magic_square::{MagicSquare, Planet};
pub use polybius::Polybius;
pub use vigenere::Vigenere;
pub use xor::Xor;

pub trait Cipher {
    fn encrypt(&self, input: &str) -> String;
    fn decrypt(&self, input: &str) -> String;
}
