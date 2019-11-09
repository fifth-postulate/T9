use std::fmt;

pub fn digits_for<S>(input: S) -> Digits
where
    S: Into<String>,
{
    let input = input.into();
    let digits: Vec<Digit> = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(Digit::from)
        .collect();
    Digits::from(digits)
}

#[derive(PartialEq, Debug)]
pub struct Digits {
    digits: Vec<Digit>,
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl From<char> for Digit {
    fn from(character: char) -> Self {
        match character {
            'a' | 'b' | 'c' | 'A' | 'B' | 'C' => Digit::Two,
            'd' | 'e' | 'f' | 'D' | 'E' | 'F' => Digit::Three,
            'g' | 'h' | 'i' | 'G' | 'H' | 'I' => Digit::Four,
            'j' | 'k' | 'l' | 'J' | 'K' | 'L' => Digit::Five,
            'm' | 'n' | 'o' | 'M' | 'N' | 'O' => Digit::Six,
            'p' | 'q' | 'r' | 's' | 'P' | 'Q' | 'R' | 'S' => Digit::Seven,
            't' | 'u' | 'v' | 'T' | 'U' | 'V' => Digit::Eight,
            'w' | 'x' | 'y' | 'z' | 'W' | 'X' | 'Y' | 'Z' => Digit::Nine,
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

macro_rules! from_for_digit {
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

from_for_digit!(1);
from_for_digit!(2);
from_for_digit!(3);
from_for_digit!(4);
from_for_digit!(5);
from_for_digit!(6);
from_for_digit!(7);
from_for_digit!(8);

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
}
