use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

enum Value
{
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
}

enum Color
{
    Spade, Diamond, Heart, Club
}

impl FromStr for Color
{
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s.len() != 1
        {
            return Err(PokerHandFromStrConversionError::ColorLenError);
        }

        let mut color_letters: HashMap<char, Color> = HashMap::new();
        color_letters.insert('C', Color::Club);
        color_letters.insert('D', Color::Diamond);
        color_letters.insert('H', Color::Heart);
        color_letters.insert('S', Color::Spade);

        match color_letters.get(&s.chars().nth(0).unwrap()) // the unwrap is safe because we know the str length is 2
        {
            Some(color) => Ok(color),
            None => Err(PokerHandFromStrConversionError::UnknownColor)
        }
    }
}

struct PokerCard
{
    value: Value,
    color: Color
}

impl FromStr for PokerCard
{
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s.len() != 2
        {
            return Err(PokerHandFromStrConversionError::CardLenError);
        }



        todo!()
    }
}

struct PokerHand<'a>(Vec<&'a PokerCard>);

enum PokerHandFromStrConversionError
{
    CardsAmountError,
    CardLenError,
    UnknownColor,
    ColorLenError
}

impl FromStr for PokerHand<'_>
{
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let split_str: Vec<&str> = s.split(" ").collect();

        if split_str.len() != 5
        {
            return Err(PokerHandFromStrConversionError::CardsAmountError);
        }



        Ok(PokerHand(vec![]))
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str>
{
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
