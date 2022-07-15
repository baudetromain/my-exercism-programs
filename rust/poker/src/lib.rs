use std::cmp::Ordering;
use std::str::FromStr;

#[macro_use]
extern crate maplit;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Spade,
    Diamond,
    Heart,
    Club,
}

#[derive(PartialEq, Debug, Clone)]
struct PokerCard {
    value: Value,
    color: Color,
}

#[derive(PartialEq, Debug, Clone)]
struct PokerHand([PokerCard; 5]);

impl FromStr for Value {
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(PokerHandFromStrConversionError::ValueLenError);
        }

        let value_chars = hashmap! {
            'A' => Value::Ace,
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            'T' => Value::Ten,
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

impl FromStr for Color {
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
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

impl FromStr for PokerCard {
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(PokerHandFromStrConversionError::CardLenError);
        }

        Ok(PokerCard {
            value: Value::from_str(s.chars().nth(0).unwrap().to_string().as_str())?,
            color: Color::from_str(s.chars().nth(1).unwrap().to_string().as_str())?,
        })
    }
}

impl FromStr for PokerHand {
    type Err = PokerHandFromStrConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_str: Vec<&str> = s.split(" ").collect();

        if split_str.len() != 5 {
            return Err(PokerHandFromStrConversionError::CardsAmountError);
        }

        split_str
            .iter()
            .map(|str_card_rep| PokerCard::from_str(str_card_rep))
            .collect::<Result<Vec<PokerCard>, PokerHandFromStrConversionError>>()
            .map(|vec| PokerHand(<[PokerCard; 5]>::try_from(vec).unwrap()))
    }
}

impl Value {
    fn ranking(&self) -> u8 {
        match self {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.ranking().cmp(&other.ranking()))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for PokerCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

#[derive(PartialEq, Debug)]
enum PokerHandFromStrConversionError {
    CardsAmountError,
    CardLenError,
    UnknownColor,
    UnknownValue,
    ColorLenError,
    ValueLenError,
}

#[derive(PartialEq)]
enum HandCombo {
    RoyalFlush, // no need to store anything cause any royal flush is equal to another
    StraightFlush(Value), // storing the highest value
    FourOfAKind(Value), // storing the value
    FullHouse([Value; 2]), // storing the 2 values
    Flush([Value; 5]), // storing the 5 values
    Straight(Value), // storing the highest value
    ThreeOfAKind(Value), // storing the value
    TwoPairs([Value; 2]), // storing the 2 values
    Pair(Value), // storing the value
    HighCard(Value), // storing the value
}

impl HandCombo {
    fn ranking(&self) -> u8 {
        match self {
            HandCombo::HighCard(_) => 1,
            HandCombo::Pair(_) => 2,
            HandCombo::TwoPairs(_) => 3,
            HandCombo::ThreeOfAKind(_) => 4,
            HandCombo::Straight(_) => 5,
            HandCombo::Flush(_) => 6,
            HandCombo::FullHouse(_) => 7,
            HandCombo::FourOfAKind(_) => 8,
            HandCombo::StraightFlush(_) => 9,
            HandCombo::RoyalFlush => 10,
        }
    }
}

impl PartialOrd for HandCombo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.ranking().cmp(&other.ranking()) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => match self {
                HandCombo::RoyalFlush => Some(Ordering::Equal),
                HandCombo::StraightFlush(self_highest) => {
                    if let HandCombo::StraightFlush(other_highest) = other {
                        return Some(self_highest.cmp(other_highest));
                    }
                    unreachable!()
                }
                HandCombo::FourOfAKind(self_highest) => {
                    if let HandCombo::FourOfAKind(other_highest) = other {
                        return Some(self_highest.cmp(other_highest));
                    }
                    unreachable!()
                }
                HandCombo::FullHouse(self_values) => {
                    if let HandCombo::FullHouse(other_values) = other {
                        let mut self_values =
                            self_values.iter().map(|n| *n).collect::<Vec<Value>>();
                        let mut other_values =
                            other_values.iter().map(|n| *n).collect::<Vec<Value>>();

                        self_values.sort_unstable();
                        other_values.sort_unstable();

                        let self_values = self_values.iter().rev().collect::<Vec<&Value>>();
                        let other_values = other_values.iter().rev().collect::<Vec<&Value>>();

                        return Some(self_values.cmp(&other_values));
                    }
                    unreachable!()
                }
                HandCombo::Flush(self_values) => {
                    if let HandCombo::Flush(other_values) = other {
                        let mut self_values =
                            self_values.iter().map(|n| *n).collect::<Vec<Value>>();
                        let mut other_values =
                            other_values.iter().map(|n| *n).collect::<Vec<Value>>();

                        self_values.sort_unstable();
                        other_values.sort_unstable();

                        let self_values = self_values.iter().rev().collect::<Vec<&Value>>();
                        let other_values = other_values.iter().rev().collect::<Vec<&Value>>();

                        return Some(self_values.cmp(&other_values));
                    }
                    unreachable!()
                }
                HandCombo::Straight(self_highest) => {
                    if let HandCombo::Straight(other_highest) = other {
                        return Some(self_highest.cmp(other_highest));
                    }
                    unreachable!()
                }
                HandCombo::ThreeOfAKind(self_highest) => {
                    if let HandCombo::ThreeOfAKind(other_highest) = other {
                        return Some(self_highest.cmp(other_highest));
                    }
                    unreachable!()
                }
                HandCombo::TwoPairs(self_values) => {
                    if let HandCombo::TwoPairs(other_values) = other {
                        let mut self_values =
                            self_values.iter().map(|n| *n).collect::<Vec<Value>>();
                        let mut other_values =
                            other_values.iter().map(|n| *n).collect::<Vec<Value>>();

                        self_values.sort_unstable();
                        other_values.sort_unstable();

                        let self_values = self_values.iter().rev().collect::<Vec<&Value>>();
                        let other_values = other_values.iter().rev().collect::<Vec<&Value>>();

                        return Some(self_values.cmp(&other_values));
                    }
                    unreachable!()
                }
                HandCombo::Pair(self_highest) => {
                    if let HandCombo::Pair(other_highest) = other {
                        return Some(self_highest.cmp(other_highest));
                    }
                    unreachable!()
                }
                HandCombo::HighCard(self_highest) => {
                    if let HandCombo::HighCard(other_highest) = other {
                        return Some(self_highest.cmp(other_highest));
                    }
                    unreachable!()
                }
            },
        }
    }
}

impl Into<Vec<PokerCard>> for PokerHand {
    fn into(self) -> Vec<PokerCard> {
        self.0.to_vec()
    }
}

impl PokerHand {
    fn find_combos(self) -> Vec<HandCombo> {
        let mut hand: Vec<PokerCard> = self.into();
        let mut combos: Vec<HandCombo> = vec![];

        while hand.len() != 0 {
            combos.push(Self::find_best_combo(&mut hand));
        }

        combos
    }

    fn find_best_combo(hand: &mut Vec<PokerCard>) -> HandCombo {
        hand.sort_by(|a, b| b.partial_cmp(a).unwrap());

        Self::find_royal_flush(hand).unwrap_or(Self::find_straight_flush(hand).unwrap_or(
            Self::find_four_of_a_kind(hand).unwrap_or(Self::find_full_house(hand).unwrap_or(
                Self::find_flush(hand).unwrap_or(
                    Self::find_straight(hand).unwrap_or(
                        Self::find_three_of_a_kind(hand).unwrap_or(
                            Self::find_two_pairs(hand).unwrap_or(
                                Self::find_pair(hand).unwrap_or(Self::find_high_card(hand)),
                            ),
                        ),
                    ),
                ),
            )),
        ))
    }

    fn find_royal_flush(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        if hand.len() != 5 {
            return None;
        }

        if hand.get(0).unwrap().value == Value::Ace
            && hand.get(1).unwrap().value == Value::King
            && hand.get(2).unwrap().value == Value::Queen
            && hand.get(3).unwrap().value == Value::Jack
            && hand.get(4).unwrap().value == Value::Ten
        {
            for i in (0..hand.len()).rev() {
                hand.remove(i);
            }

            return Some(HandCombo::RoyalFlush);
        }

        None
    }

    fn find_straight_flush(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        if hand.len() != 5 {
            return None;
        }

        if hand.get(0).unwrap().value.ranking() == hand.get(1).unwrap().value.ranking() + 1
            && hand.get(1).unwrap().value.ranking() == hand.get(2).unwrap().value.ranking() + 1
            && hand.get(2).unwrap().value.ranking() == hand.get(3).unwrap().value.ranking() + 1
            && hand.get(3).unwrap().value.ranking() == hand.get(4).unwrap().value.ranking() + 1
            && [
                hand.get(0).unwrap().color,
                hand.get(1).unwrap().color,
                hand.get(2).unwrap().color,
                hand.get(3).unwrap().color,
                hand.get(4).unwrap().color,
            ]
            .iter()
            .filter(|color| color != &&hand.get(0).unwrap().color)
            .count()
                == 0
        {
            for i in (1..hand.len()).rev() {
                hand.remove(i);
            }

            let highest_card = hand.remove(0);
            return Some(HandCombo::StraightFlush(highest_card.value));
        }

        None
    }

    fn find_four_of_a_kind(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        if hand.len() < 4 {
            return None;
        }

        for i in 0..2 {
            let value = hand.get(i).unwrap().value;
            if hand
                .iter()
                .filter(|card| card.value == value)
                .count()
                == 4
            {
                let mut indexes = hand.iter()
                    .enumerate()
                    .filter(|(_, card)| card.value == value)
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>();

                indexes.sort();

                for index in indexes.iter().rev()
                {
                    hand.remove(*index);
                }
            }
        }

        todo!("find the four of a kind if there is one")
    }

    fn find_full_house(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        unimplemented!()
    }

    fn find_flush(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        unimplemented!()
    }

    fn find_straight(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        unimplemented!()
    }

    fn find_three_of_a_kind(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        unimplemented!()
    }

    fn find_two_pairs(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        unimplemented!()
    }

    fn find_pair(hand: &mut Vec<PokerCard>) -> Option<HandCombo> {
        unimplemented!()
    }

    fn find_high_card(hand: &mut Vec<PokerCard>) -> HandCombo {
        unimplemented!()
    }
}

#[test]
fn basic_success() {
    assert_eq!(
        "2S 4S 7H AC JH".parse::<PokerHand>(),
        Ok(PokerHand {
            0: [
                PokerCard {
                    color: Color::Spade,
                    value: Value::Two
                },
                PokerCard {
                    color: Color::Spade,
                    value: Value::Four
                },
                PokerCard {
                    color: Color::Heart,
                    value: Value::Seven
                },
                PokerCard {
                    color: Color::Club,
                    value: Value::Ace
                },
                PokerCard {
                    color: Color::Heart,
                    value: Value::Jack
                },
            ]
        })
    )
}

#[test]
fn empty() {
    assert_eq!(
        "".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::CardsAmountError)
    )
}

#[test]
fn not_enough_cards() {
    assert_eq!(
        "2S 4S 7H AC".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::CardsAmountError)
    )
}

#[test]
fn too_big_card() {
    assert_eq!(
        "2S 4S 7H AC JHH".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::CardLenError)
    )
}

#[test]
fn too_small_card() {
    assert_eq!(
        "2S 4S 7H AC J".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::CardLenError)
    )
}

#[test]
fn wrong_color() {
    assert_eq!(
        "2S 4S 7H AC JX".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::UnknownColor)
    )
}

#[test]
fn wrong_value() {
    assert_eq!(
        "2S 4S 7H AC ZH".parse::<PokerHand>(),
        Err(PokerHandFromStrConversionError::UnknownValue)
    )
}

#[test]
fn different_combos() {
    assert_eq!(
        HandCombo::RoyalFlush.partial_cmp(&HandCombo::Pair(Value::Five)),
        Some(Ordering::Greater)
    )
}

#[test]
fn different_combos2() {
    assert_eq!(
        HandCombo::ThreeOfAKind(Value::Three)
            .partial_cmp(&HandCombo::TwoPairs([Value::Five, Value::Eight])),
        Some(Ordering::Greater)
    )
}

#[test]
fn same_combos_simple_element() {
    assert_eq!(
        HandCombo::Pair(Value::Three).partial_cmp(&HandCombo::Pair(Value::Seven)),
        Some(Ordering::Less)
    )
}

#[test]
fn same_combos_two_elements() {
    assert_eq!(
        HandCombo::TwoPairs([Value::Three, Value::Nine])
            .partial_cmp(&HandCombo::TwoPairs([Value::Five, Value::Seven])),
        Some(Ordering::Greater)
    )
}

#[test]
fn same_combos_two_elements_equal() {
    assert_eq!(
        HandCombo::FullHouse([Value::Three, Value::Nine])
            .partial_cmp(&HandCombo::FullHouse([Value::Nine, Value::Three])),
        Some(Ordering::Equal)
    )
}

#[test]
fn same_combos_five_elements() {
    assert_eq!(
        HandCombo::Flush([
            Value::Two,
            Value::Five,
            Value::Seven,
            Value::Queen,
            Value::Ace
        ])
        .partial_cmp(&HandCombo::Flush([
            Value::Two,
            Value::Five,
            Value::Eight,
            Value::Queen,
            Value::Ace
        ])),
        Some(Ordering::Less)
    )
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
