#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
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

fn main() {
    let deck = Deck::new();

    println!("Here's your deck: {:#?}", deck);
}
