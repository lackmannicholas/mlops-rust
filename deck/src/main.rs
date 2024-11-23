#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    // array of suits for a deck of cards
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];

    // array of ranks for a deck of cards
    let ranks = [
        "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
    ];

    let deck = Deck { cards: Vec::new() };

    // double for loop to create a deck of cards
    for suit in suits {
        for rank in ranks {
            // add to suit and rank combination to the deck
            deck.cards.push(format!("{} of {}", rank, suit));
        }
    }

    

    println!("Here's your deck: {:?}", deck);
}
