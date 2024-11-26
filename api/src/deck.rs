use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Deck {
    cards: Vec<String>,
}

impl Deck {
    pub fn new() -> Deck {
        // array of suits for a deck of cards
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];

        // array of ranks for a deck of cards
        let ranks = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];

        // create a new cards vector of strings
        let mut cards: Vec<String> = Vec::new();

        // double for loop to create a deck of cards
        for suit in suits {
            for rank in ranks {
                // add to suit and rank combination to the deck
                cards.push(format!("{} of {}", rank, suit));
            }
        }

        Deck { cards }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_contains_card() {
        let deck = Deck::new();
        assert!(deck.cards.contains(&"Ace of Hearts".to_string()));
        assert!(deck.cards.contains(&"King of Spades".to_string()));
    }
}