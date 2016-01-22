extern crate cards;

use std::cmp::Ordering;
use std::fmt;

use cards::deck::Deck;
use cards::card::Suit;
use cards::card::Value;

struct Card(cards::card::Card);

fn suit_symbol(suit: &Suit) -> &'static str {
/*
    match *suit {
        Suit::Spades => "\u{2664}",
        Suit::Hearts => "\u{2661}",
        Suit::Diamonds => "\u{2662}",
        Suit::Clubs => "\u{2667}",
    }
// */
    match *suit {
        Suit::Spades => "\u{2660}",
        Suit::Hearts => "\u{2665}",
        Suit::Diamonds => "\u{2666}",
        Suit::Clubs => "\u{2663}",
    }
// */
}

fn value_string(val: &Value) -> &'static str {
    match *val {
        Value::Two => "2",
        Value::Three => "3",
        Value::Four => "4",
        Value::Five => "5",
        Value::Six => "6",
        Value::Seven => "7",
        Value::Eight => "8",
        Value::Nine => "9",
        Value::Ten => "T",
        Value::Jack => "J",
        Value::Queen => "Q",
        Value::King => "K",
        Value::Ace => "A",
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", value_string(&self.0.value), suit_symbol(&self.0.suit))
    }
}

fn main() {
    let mut deck = Deck::new_shuffled();
    let mut tableau = vec![
        deck.draw_n(1).ok().unwrap(),
        deck.draw_n(2).ok().unwrap(),
        deck.draw_n(3).ok().unwrap(),
        deck.draw_n(4).ok().unwrap(),
        deck.draw_n(5).ok().unwrap(),
        deck.draw_n(6).ok().unwrap(),
        deck.draw_n(7).ok().unwrap(),
    ];
    for i in 0..7 {
        for j in 0..7 {
            match (tableau[j].len() - 1).cmp(&(i as usize)) {
                Ordering::Greater => print!("??"),
                Ordering::Equal   => print!("{}", Card(tableau[i][j])),
                Ordering::Less    => print!("  "),
            };
            print!{" "};
        }
        println!("");
    }
}
