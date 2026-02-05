use std::collections::HashMap;

use uuid::Uuid;

use crate::core::{development_card::DevelopmentCard, resource::{self, Resource}};

// player struct houses the player information
pub struct Player {
    id: Uuid,
    name: String,
    color: PlayerColor,
    pub playerhand: PlayerHand
}

impl Player {
    pub fn get_resource_count(&self, resource: Resource) -> u8 {
        let resource_count = self.playerhand.resources.get(&resource);
        match resource_count {
            Some(n) => *n,
            None => 0
        }
    }
}

#[derive(Clone)]
pub struct PlayerHand {
    pub resources: HashMap<Resource, u8>,
    pub road_pieces: u8,
    pub settlement_pieces: u8,
    pub city_pieces: u8,
    pub development_cards: Vec<DevelopmentCard>,
    pub icbm_pieces: u8
}

impl PlayerHand {
    // defines a starting playerhand
    fn new() -> Self {
        Self {
            resources: HashMap::new(),
            road_pieces: 4,
            settlement_pieces: 2,
            city_pieces: 0,
            development_cards: Vec::new(),
            icbm_pieces: 1
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlayerColor {
    Red,
    Blue,
    Orange,
    White
}

// tests for the player struct
#[cfg(test)]
mod tests {
    use crate::core::rules::{can_afford_city, can_afford_development_card, can_afford_icbm, can_afford_road};

    use super::*;

    #[test]
    fn can_afford() {
        let default_resources: HashMap<Resource, u8> = HashMap::from([
            (Resource::Brick, 2),
            (Resource::Ore, 3),
            (Resource::Uranium, 2),
            (Resource::Wheat, 2),
            (Resource::Wood, 0),
            (Resource::Wool, 2)
        ]);

        let test_hand = PlayerHand {
            resources: default_resources,
            road_pieces: 4,
            settlement_pieces: 2,
            city_pieces: 0,
            development_cards: Vec::new(),
            icbm_pieces: 1
        };

        let player1 = Player {
            id: Uuid::new_v4(),
            name: "player 1".to_string(),
            color: PlayerColor::Blue,
            playerhand: test_hand.clone()
        };

        assert!(!can_afford_road(&player1));
        assert!(can_afford_city(&player1));
        assert!(can_afford_icbm(&player1));
        assert!(can_afford_development_card(&player1));
    }
}