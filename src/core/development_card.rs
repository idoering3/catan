use rand::seq::SliceRandom;
use std::iter::repeat;

#[derive(Clone, Copy)]
pub enum DevelopmentCard {
    Monopoly,
    RoadBuilding,
    VictoryPoint,
    Knight,
    YearOfPlenty,

    TowerOfBabel,
    Spy,
    KingArthur,
    UssIowa
}

pub struct DevDeck {
    cards: Vec<DevelopmentCard>
}

impl DevDeck {
    fn new() -> Self {
        let mut cards = Vec::new();

        // standard game cards
        cards.extend(repeat(DevelopmentCard::Knight).take(14));
        cards.extend(repeat(DevelopmentCard::VictoryPoint).take(5));
        cards.extend(repeat(DevelopmentCard::RoadBuilding).take(2));
        cards.extend(repeat(DevelopmentCard::YearOfPlenty).take(2));
        cards.extend(repeat(DevelopmentCard::Monopoly).take(2));
        
        // custom cards
        cards.extend(repeat(DevelopmentCard::TowerOfBabel).take(2));
        cards.extend(repeat(DevelopmentCard::Spy).take(2));
        cards.extend(repeat(DevelopmentCard::KingArthur).take(1));
        cards.extend(repeat(DevelopmentCard::UssIowa).take(1));
        

        let mut rng = rand::rng();
        cards.shuffle(&mut rng);

        Self { cards }
    }
    fn draw(&mut self) -> Option<DevelopmentCard> {
        self.cards.pop()
    }
}