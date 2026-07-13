use std::io;

use uuid::Uuid;

use crate::core::{bank::Bank, board::Board, development_card::DevDeck, player::player::{Player, PlayerColor, PlayerHand}};

enum GamePhase {
    Setup,
    Roll,
    Trade,
    Build    
}

pub struct Game {
    players: Vec<Player>,
    current_player: usize,

    board: Board,

    dev_deck: DevDeck,
    bank: Bank,

    phase: GamePhase,
    turn_number: u32
}
