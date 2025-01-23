use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>, //Vectors---- like arrays but can change size
}

impl Deck {
    fn new() -> Self {
        /** Inherit implementaiton a.k.a. Add a function to a struct....can define methods and functions
        - Associated functions ---- not tied to an instance
        - Methods -----tied to an instance

        */
        //Arrays vs Vectors performance difference is tiny and is a matter of communication
        let suits = ["Hearts", "Spades", "Diamonds"];
        let suits2 = vec!["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        /*
        - Variables are immutable by default. Technical an immutable variable is called a Binding
         */
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        // println!("Heres your deck: {:?}", cards);
        // return Deck { cards };
        // return deck;
        // deck
        Deck { cards } //Implicit return - auto return the last expression
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> { //TODO: See diagram for list of number types in Rust
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    println!("Heres your deck: {:?}", deck);
    deck.shuffle();
    let cards = deck.deal(3);// Consider error handling.

    let deck2 = Deck { cards: vec![] };
    let deck2 = Deck { cards: Vec::new() };

    println!("Heres your deck shuffled: {:?}", deck);
    println!("Heres your cards: {:?}", cards);
}