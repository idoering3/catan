
// we want to implement an axial coordinate system for the board (cube but minus a dimension. You can easily find the third dimension if needed)

use std::collections::HashMap;

use crate::core::tile::Tile;

// this is JUST for hexes. for roads and settlements we use separate coordinates
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32
}

impl HexCoord {
    pub fn neighbors(&self) -> [HexCoord; 6] {
        // return going ccw from right
        [
            HexCoord { q: self.q + 1, r: self.r     },
            HexCoord { q: self.q + 1, r: self.r - 1 },
            HexCoord { q: self.q    , r: self.r - 1 },
            HexCoord { q: self.q - 1, r: self.r     },
            HexCoord { q: self.q - 1, r: self.r + 1 },
            HexCoord { q: self.q    , r: self.r + 1 }
        ]
    }
}

// implement the board here. We use the tile.rs file for the tile.

#[derive(Clone, Debug)]
pub struct Board {
    // we look up the hashmap by coordinate, e.g. (r, q)
    // so we can do (0, 0) in the hashmap and get the tile.
    // we can also iterate through the hashmap of tiles (e.g. check dice roll)
    pub tiles: HashMap<HexCoord, Tile>
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();
        let radius: i32 = 2;

        for q in -radius..=radius {
            for r in -radius..=radius {
                if (q + r).abs() <= radius {
                    let coord = HexCoord{ q, r };
                    tiles.insert(coord.clone(), Tile {
                        coord,
                        resource: None,
                        tile_type: None,
                        dice_number: None,
                    });
                }
            }
        }
        Board{tiles}
    }

    pub fn tile_neighbors(&self, coord: &HexCoord) -> Result<HashMap<HexCoord, &Tile>, String> {
        let Some(center_tile) = self.tiles.get(coord) else {
            return Err("The provided coordinate has no associated hex tile".to_string());
        };

        let neighbor_coords = center_tile.coord.neighbors();

        let mut neighbors = HashMap::new();

        for coord in neighbor_coords {
            if let Some(tile) = self.tiles.get(&coord) {
                neighbors.insert(tile.coord, tile);
            }
        }

        Ok(neighbors)
    }


}

// tests
mod tests {
    use super::*;

    #[test]
    fn instantiate_board() {
        let board = Board::new();
        let tiles = board.clone().tiles;
        let tiles_count = tiles.len();
        // yay unwrap for testing
        let center_tile_neighbors = board.tile_neighbors(&HexCoord { q: 0, r: 0 }).unwrap();
        let center_tile_neighbors_count = center_tile_neighbors.len();
        
        let corner_tile_neighbors = board.tile_neighbors(&HexCoord { q: 0, r: -2 }).unwrap();
        let corner_tile_neighbors_count = corner_tile_neighbors.len();
        
        assert_eq!(tiles_count, 19);
        assert_eq!(center_tile_neighbors_count, 6);
        assert_eq!(corner_tile_neighbors_count, 3);
    }
}