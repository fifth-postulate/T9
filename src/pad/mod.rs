//! Converts from a word to keypad digits
//!
//! ```
//! # extern crate t9;
//! # use t9::pad;
//! # fn check_that_rust_corresponds_to_7878() {
//! let digits = pad::digits_for("rust");
//! let output = digits.to_string();
//! assert_eq!(output, String::from("7878"));
//! # }
//! ```
use std::fmt;

/// Takes an input word and returns the corresponding T9 digits needed to return the word.
///
/// See [wikipedia](https://en.wikipedia.org/wiki/T9_%28predictive_text%29) for more information about T9.
pub fn digits_for<S>(input: S) -> Digits
where
    S: Into<String>,
{
    let input = input.into();
    let digits: Vec<Digit> = input
        .chars()
        .filter(|c| c.is_ascii())
        .map(Digit::from)
        .collect();
    Digits::from(digits)
}

/// A wrapper for a sequence of `Digit`s.
#[derive(PartialEq, Debug)]
pub struct Digits {
    digits: Vec<Digit>,
}

impl Digits {
    /// Returns the first `Digit` of this sequence.
    ///
    /// ```
    /// # extern crate t9;
    /// # use t9::pad::{Digit, Digits};
    /// # fn head_should_work_correctly() {
    /// assert_eq!(Digits::from(vec![Digit::Two, Digit::One]).head(), Some(&Digit::Two));
    /// assert_eq!(Digits::from(vec![]).head(), None);
    /// # }
    /// ```
    ///
    /// Note that the sequence can be empty. In that case it will return `None`.
    pub fn head(&self) -> Option<&Digit> {
        self.digits.get(0)
    }

    /// Returns the tail of this sequence.
    ///
    /// ```
    /// # extern crate t9;
    /// # use t9::pad::{Digit, Digits};
    /// # fn tail_should_work_correctly() {
    /// assert_eq!(Digits::from(vec![Digit::Two, Digit::One]).tail(), Digits::from(vec![Digit::One]));
    /// assert_eq!(Digits::from(vec![]).tail(), Digits::from(vec![]));
    /// }
    /// ```
    pub fn tail(&self) -> Digits {
        Self {
            digits: self.digits[1..].to_vec(),
        }
    }
}

impl From<Vec<Digit>> for Digits {
    fn from(digits: Vec<Digit>) -> Self {
        Self { digits }
    }
}

impl fmt::Display for Digits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in &self.digits {
            write!(f, "{}", digit)?
        }
        write!(f, "")
    }
}

/// A digit that can be used with T9.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Digit {
    /// Corresponds with button 1, not used for T9, kept for completeness
    One,
    /// Corresponds with button 2
    Two,
    /// Corresponds with button 3
    Three,
    /// Corresponds with button 4
    Four,
    /// Corresponds with button 5
    Five,
    /// Corresponds with button 6
    Six,
    /// Corresponds with button 7
    Seven,
    /// Corresponds with button 8
    Eight,
    /// Corresponds with button 9
    Nine,
    /// Corresponds with button 0, not used for T9, kept for completeness
    Zero,
}

impl From<char> for Digit {
    fn from(character: char) -> Self {
        match character {
            '1' => Digit::One,
            '2' | 'a' | 'b' | 'c' | 'A' | 'B' | 'C' => Digit::Two,
            '3' | 'd' | 'e' | 'f' | 'D' | 'E' | 'F' => Digit::Three,
            '4' | 'g' | 'h' | 'i' | 'G' | 'H' | 'I' => Digit::Four,
            '5' | 'j' | 'k' | 'l' | 'J' | 'K' | 'L' => Digit::Five,
            '6' | 'm' | 'n' | 'o' | 'M' | 'N' | 'O' => Digit::Six,
            '7' | 'p' | 'q' | 'r' | 's' | 'P' | 'Q' | 'R' | 'S' => Digit::Seven,
            '8' | 't' | 'u' | 'v' | 'T' | 'U' | 'V' => Digit::Eight,
            '9' | 'w' | 'x' | 'y' | 'z' | 'W' | 'X' | 'Y' | 'Z' => Digit::Nine,
            _ => Digit::Zero,
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let character = match self {
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
            Digit::Zero => '_',
        };
        write!(f, "{}", character)
    }
}

macro_rules! from_for_digits {
    ($n:expr) => {
        impl From<[Digit; $n]> for Digits {
            fn from(digits: [Digit; $n]) -> Self {
                Self {
                    digits: digits.to_vec(),
                }
            }
        }
    };
}

from_for_digits!(1);
from_for_digits!(2);
from_for_digits!(3);
from_for_digits!(4);
from_for_digits!(5);
from_for_digits!(6);
from_for_digits!(7);
from_for_digits!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_should_be_able_to_be_constructed_from_arrays() {
        assert_eq!(Digits::from([Digit::One]), Digits::from(vec![Digit::One]));
        assert_eq!(
            Digits::from([Digit::One, Digit::Two]),
            Digits::from(vec![Digit::One, Digit::Two])
        );
        assert_eq!(
            Digits::from([Digit::One, Digit::Two, Digit::Three]),
            Digits::from(vec![Digit::One, Digit::Two, Digit::Three])
        );
        assert_eq!(
            Digits::from([Digit::One, Digit::Two, Digit::Three, Digit::Four]),
            Digits::from(vec![Digit::One, Digit::Two, Digit::Three, Digit::Four])
        );
    }

    #[test]
    fn digit_should_display_correctly() {
        for (digit, expected) in [
            (Digit::One, "1"),
            (Digit::Two, "2"),
            (Digit::Three, "3"),
            (Digit::Four, "4"),
            (Digit::Five, "5"),
            (Digit::Six, "6"),
            (Digit::Seven, "7"),
            (Digit::Eight, "8"),
            (Digit::Nine, "9"),
            (Digit::Zero, "_"),
        ]
        .iter()
        {
            assert_eq!(digit.to_string(), *expected);
        }
    }

    #[test]
    fn character_should_be_transformed_into_digit() {
        for (character, expected) in [
            ('1', Digit::One),
            ('2', Digit::Two),
            ('3', Digit::Three),
            ('4', Digit::Four),
            ('5', Digit::Five),
            ('6', Digit::Six),
            ('7', Digit::Seven),
            ('8', Digit::Eight),
            ('9', Digit::Nine),
            ('0', Digit::Zero),
            ('a', Digit::Two),
            ('b', Digit::Two),
            ('c', Digit::Two),
            ('d', Digit::Three),
            ('e', Digit::Three),
            ('f', Digit::Three),
            ('g', Digit::Four),
            ('h', Digit::Four),
            ('i', Digit::Four),
            ('j', Digit::Five),
            ('k', Digit::Five),
            ('l', Digit::Five),
            ('m', Digit::Six),
            ('n', Digit::Six),
            ('o', Digit::Six),
            ('p', Digit::Seven),
            ('q', Digit::Seven),
            ('r', Digit::Seven),
            ('s', Digit::Seven),
            ('t', Digit::Eight),
            ('u', Digit::Eight),
            ('v', Digit::Eight),
            ('w', Digit::Nine),
            ('x', Digit::Nine),
            ('y', Digit::Nine),
            ('z', Digit::Nine),
            ('A', Digit::Two),
            ('B', Digit::Two),
            ('C', Digit::Two),
            ('D', Digit::Three),
            ('E', Digit::Three),
            ('F', Digit::Three),
            ('G', Digit::Four),
            ('H', Digit::Four),
            ('I', Digit::Four),
            ('J', Digit::Five),
            ('K', Digit::Five),
            ('L', Digit::Five),
            ('M', Digit::Six),
            ('N', Digit::Six),
            ('O', Digit::Six),
            ('P', Digit::Seven),
            ('Q', Digit::Seven),
            ('R', Digit::Seven),
            ('S', Digit::Seven),
            ('T', Digit::Eight),
            ('U', Digit::Eight),
            ('V', Digit::Eight),
            ('W', Digit::Nine),
            ('X', Digit::Nine),
            ('Y', Digit::Nine),
            ('Z', Digit::Nine),
        ]
        .iter()
        {
            assert_eq!(Digit::from(*character), *expected);
        }
    }

    #[test]
    fn digit_for_should_return_with_correct_digits() {
        let actual = digits_for("rust");

        let expected = Digits::from([Digit::Seven, Digit::Eight, Digit::Seven, Digit::Eight]);
        assert_eq!(actual, expected);
    }
}
