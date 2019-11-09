use std::fmt;

pub fn numbers_for(_input: &str) -> Digits {
    Digits::from([Digit::Seven, Digit::Eight, Digit::Seven, Digit::Eight])
}

pub struct Digits {
    digits: Vec<Digit>,
}

impl From<[Digit; 4]> for Digits {
    fn from(digits: [Digit; 4]) -> Self {
        Self {
            digits: digits.to_vec(),
        }
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

#[derive(Copy, Clone)]
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
            Digit::Zero => '0',
        };
        write!(f, "{}", character)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_one_should_display_correctly() {
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
            (Digit::Zero, "0"),
        ]
        .iter()
        {
            assert_eq!(digit.to_string(), *expected);
        }
    }
}
