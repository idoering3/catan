use crate::core::board::{Board, HexCoord};

mod core;

fn main() {
    println!("Catan Testing");

    let board = Board::new();
    let center_neighbors = board.tile_neighbors(&HexCoord{q: 0, r: 0});

    println!("{:?}", board);
    println!("{:?}", center_neighbors);
}
