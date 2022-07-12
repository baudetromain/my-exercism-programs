use std::cmp::Ordering;
use std::str::FromStr;

#[macro_use] extern crate maplit;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(Clone, Copy, PartialEq, Debug)]
enum Value
{
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Jack, Queen, King, Ace
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Color
{
    Spade, Diamond, Heart, Club
}

#[derive(PartialEq, Debug)]
struct PokerCard
{
    value: Value,
    color: Color
}

#[derive(PartialEq, Debug)]
struct PokerHand([PokerCard ; 5]);

impl FromStr for Value
{
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s.len() != 1
        {
            return Err(PokerHandFromStrConversionError::ValueLenError);
        }

        let value_chars = hashmap!{
            'A' => Value::Ace,
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            'J' => Value::Jack,
            'Q' => Value::Queen,
            'K' => Value::King,
        };

        match value_chars.get(&s.chars().nth(0).unwrap()) // the unwrap is safe because we know the str length is 1
        {
            Some(color) => Ok(*color), // won't work without the Copy trait cause we would not be able to move the color out of the HashMap
            None => Err(PokerHandFromStrConversionError::UnknownValue)
        }
    }
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

        let color_chars = hashmap! {
            'C' => Color::Club,
            'D' => Color::Diamond,
            'H' => Color::Heart,
            'S' => Color::Spade,
        };

        match color_chars.get(&s.chars().nth(0).unwrap()) // the unwrap is safe because we know the str length is 2
        {
            Some(color) => Ok(*color), // won't work without the Copy trait cause we would not be able to move the color out of the HashMap
            None => Err(PokerHandFromStrConversionError::UnknownColor)
        }
    }
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

        Ok(PokerCard
        {
            value: Value::from_str(s.chars().nth(0).unwrap().to_string().as_str())?,
            color: Color::from_str(s.chars().nth(1).unwrap().to_string().as_str())?,
        })
    }
}

impl FromStr for PokerHand
{
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let split_str: Vec<&str> = s.split(" ").collect();

        if split_str.len() != 5
        {
            return Err(PokerHandFromStrConversionError::CardsAmountError);
        }

        split_str.iter()
            .map(|str_card_rep| PokerCard::from_str(str_card_rep))
            .collect::<Result<Vec<PokerCard>, PokerHandFromStrConversionError>>()
            .map(|vec| PokerHand(<[PokerCard; 5]>::try_from(vec).unwrap()))
    }
}

#[derive(PartialEq, Debug)]
enum PokerHandFromStrConversionError
{
    CardsAmountError,
    CardLenError,
    UnknownColor,
    UnknownValue,
    ColorLenError,
    ValueLenError
}

#[derive(PartialEq)]
enum HandCombo
{
    RoyalFlush,             // no need to store anything cause any royal flush is equal to another
    StraightFlush(u8),      // storing the highest
    FourOfAKind(u8),        // storing the value
    FullHouse([u8 ; 2]),    // storing the 2 values
    Flush([u8 ; 5]),        // storing the 5 values
    Straight(u8),           // storing the highest value
    ThreeOfAKind(u8),       // storing the value
    TwoPairs([u8 ; 2]),     // storing the 2 values
    Pair(u8),               // storing the value
    HighCard(u8)            // storing the value
}

impl HandCombo
{
    fn ranking(&self) -> u8
    {
        match self
        {
            HandCombo::HighCard(_) => 1,
            HandCombo::Pair(_) => 2,
            HandCombo::TwoPairs(_) => 3,
            HandCombo::ThreeOfAKind(_) => 4,
            HandCombo::Straight(_) => 5,
            HandCombo::Flush(_) => 6,
            HandCombo::FullHouse(_) => 7,
            HandCombo::FourOfAKind(_) => 8,
            HandCombo::StraightFlush(_) => 9,
            HandCombo::RoyalFlush => 10
        }
    }
}

impl PartialOrd for HandCombo
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        match self.ranking().cmp(&other.ranking())
        {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal =>
            {
                match self
                {
                    HandCombo::RoyalFlush => Some(Ordering::Equal),
                    HandCombo::StraightFlush(self_highest) =>
                    {
                        if let HandCombo::StraightFlush(other_highest) = other
                        {
                            return Some(self_highest.cmp(other_highest))
                        }
                        None
                    },
                    HandCombo::FourOfAKind(self_highest) =>
                    {
                        if let HandCombo::FourOfAKind(other_highest) = other
                        {
                            return Some(self_highest.cmp(other_highest))
                        }
                        None
                    },
                    HandCombo::FullHouse(self_values) =>
                    {
                        if let HandCombo::FullHouse(other_values) = other
                        {
                            let mut self_values = self_values.iter().map(|n| *n).collect::<Vec<u8>>();
                            let mut other_values = other_values.iter().map(|n| *n).collect::<Vec<u8>>();

                            self_values.sort();
                            other_values.sort();

                            return if let Ordering::Equal = self_values.get(0).cmp(&other_values.get(0))
                            {
                                Some(self_values.get(1).cmp(&other_values.get(1)))
                            }
                            else
                            {
                                Some(self_values.get(0).cmp(&other_values.get(0)))
                            }
                        }
                        None
                    },
                    _ => None // TODO
                }
            }
        }
    }
}

#[test]
fn basic_success()
{
    assert_eq!(
        "2S 4S 7H AC JH".parse::<PokerHand>(),
        Ok(PokerHand{ 0: [
            PokerCard { color: Color::Spade, value: Value::Two },
            PokerCard { color: Color::Spade, value: Value::Four },
            PokerCard { color: Color::Heart, value: Value::Seven },
            PokerCard { color: Color::Club, value: Value::Ace },
            PokerCard { color: Color::Heart, value: Value::Jack },
        ] })
    );
}

#[test]
fn empty()
{
    assert_eq!(
        "".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::CardsAmountError)
    )
}

#[test]
fn not_enough_cards()
{
    assert_eq!(
        "2S 4S 7H AC".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::CardsAmountError)
    )
}

#[test]
fn too_big_card()
{
    assert_eq!(
        "2S 4S 7H AC JHH".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::CardLenError)
    )
}

#[test]
fn too_small_card()
{
    assert_eq!(
        "2S 4S 7H AC J".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::CardLenError)
    )
}

#[test]
fn wrong_color()
{
    assert_eq!(
        "2S 4S 7H AC JX".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::UnknownColor)
    )
}

#[test]
fn wrong_value()
{
    assert_eq!(
        "2S 4S 7H AC ZH".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::UnknownValue)
    )
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str>
{
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
