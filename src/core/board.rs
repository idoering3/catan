
// we want to implement an axial coordinate system for the board (cube but minus a dimension. You can easily find the third dimension if needed)

use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::core::tile::Tile;

// this is JUST for hexes. for roads and settlements we use separate coordinates
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VertexCoord {
    pub x: i32,
    pub y: i32,
}

impl VertexCoord {
    pub fn adjacent_hexes(&self, board: &Board) -> Vec<HexCoord> {
        // filter by adjacent hexes. 
        board.tiles.keys()
            .filter(|hex_coord| hex_coord.vertices().contains(self))
            .copied()
            .collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EdgeCoord {
    pub x: i32,
    pub y: i32,
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

    pub fn vertices(&self) -> [VertexCoord; 6] {
        let q = self.q;
        let r = self.r;
        
        [
            VertexCoord { x: 2*q + r,     y: 2*r - 1 },   // Top
            VertexCoord { x: 2*q + r + 1, y: 2*r     },   // Top-right
            VertexCoord { x: 2*q + r + 1, y: 2*r + 1 },   // Bottom-right
            VertexCoord { x: 2*q + r,     y: 2*r + 2 },   // Bottom
            VertexCoord { x: 2*q + r - 1, y: 2*r + 1 },   // Bottom-left
            VertexCoord { x: 2*q + r - 1, y: 2*r     },   // Top-left
        ]
    }

    pub fn edges(&self) -> [EdgeCoord; 6] {
        let q = self.q;
        let r = self.r;
        
        [
            EdgeCoord { x: 4*q + 2*r + 1, y: 4*r - 1 },   // NE
            EdgeCoord { x: 4*q + 2*r + 2, y: 4*r + 1 },   // E (vertical edge)
            EdgeCoord { x: 4*q + 2*r + 1, y: 4*r + 3 },   // SE
            EdgeCoord { x: 4*q + 2*r - 1, y: 4*r + 3 },   // SW
            EdgeCoord { x: 4*q + 2*r - 2, y: 4*r + 1 },   // W (vertical edge)
            EdgeCoord { x: 4*q + 2*r - 1, y: 4*r - 1 },   // NW
        ]
    }
}

// implement the board here. We use the tile.rs file for the tile.

#[derive(Clone, Debug)]
pub struct Board {
    // we look up the hashmap by coordinate, e.g. (r, q)
    // so we can do (0, 0) in the hashmap and get the tile.
    // we can also iterate through the hashmap of tiles (e.g. check dice roll)
    pub tiles: HashMap<HexCoord, Tile>,
    pub vertices: HashSet<VertexCoord>,
    pub edges: HashSet<EdgeCoord>,
}

impl Board {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();
        let radius: i32 = 2;

        for q in -radius..=radius {
            for r in -radius..=radius {
                if (q + r).abs() <= radius {
                    let coord = HexCoord{ q, r };
                    tiles.insert(coord, Tile {
                        coord,
                        resource: None,
                        tile_type: None,
                        dice_number: None,
                    });
                }
            }
        }

        let mut vertices = HashSet::new();
        let mut edges = HashSet::new();
        for hex in tiles.values() {
            for vertex in hex.coord.vertices() {
                vertices.insert(vertex);
            }
            for edge in hex.coord.edges() {
                edges.insert(edge);
            }
        }

        Board{tiles, vertices, edges}
    }

    pub fn generate_vertices(&mut self) -> HashSet<VertexCoord> {
        let mut vertices: HashSet<VertexCoord> = HashSet::new();

        for hex in self.tiles.values() {
            for vertex in hex.coord.vertices() {
                vertices.insert(vertex);
            }
        }   
        self.vertices = vertices.clone();
        vertices
    }

    pub fn generate_edges(&mut self) -> HashSet<EdgeCoord> {
        let mut edges: HashSet<EdgeCoord> = HashSet::new();

        for hex in self.tiles.values() {
            for edge in hex.coord.edges() {
                edges.insert(edge);
            }
        }   
        self.edges = edges.clone();
        edges
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

    #[test]
    fn board_tiles_and_vertices_count() {
        let board = Board::new();
        
        assert_eq!(board.tiles.len(), 19);
        assert_eq!(board.vertices.len(), 54);
        assert_eq!(board.edges.len(), 72);
    }

    #[test]
    fn test_adjacent_hexes_share_correct_vertices_and_edges() {
        let board = Board::new();
        
        // Test that every pair of adjacent hexes shares exactly 2 vertices and 1 edge
        for (coord, _tile) in &board.tiles {
            let neighbors = board.tile_neighbors(coord).unwrap();
            
            for (neighbor_coord, _neighbor_tile) in neighbors {
                let my_verts: HashSet<_> = coord.vertices().into_iter().collect();
                let neighbor_verts: HashSet<_> = neighbor_coord.vertices().into_iter().collect();
                let shared_verts: Vec<_> = my_verts.intersection(&neighbor_verts).collect();
                
                let my_edges: HashSet<_> = coord.edges().into_iter().collect();
                let neighbor_edges: HashSet<_> = neighbor_coord.edges().into_iter().collect();
                let shared_edges: Vec<_> = my_edges.intersection(&neighbor_edges).collect();
                
                assert_eq!(shared_verts.len(), 2, 
                    "Hexes {:?} and {:?} should share exactly 2 vertices, found {}", 
                    coord, neighbor_coord, shared_verts.len());
                assert_eq!(shared_edges.len(), 1, 
                    "Hexes {:?} and {:?} should share exactly 1 edge, found {}", 
                    coord, neighbor_coord, shared_edges.len());
            }
        }
    }

    #[test]
    fn test_each_vertex_touches_three_hexes() {
        let board = Board::new();
        
        // Count how many hexes touch each vertex
        let mut vertex_hex_count: HashMap<VertexCoord, usize> = HashMap::new();
        
        for (coord, _tile) in &board.tiles {
            for vertex in coord.vertices() {
                *vertex_hex_count.entry(vertex).or_insert(0) += 1;
            }
        }
        
        // Most vertices should be touched by exactly 3 hexes (except border vertices which touch 1 or 2)
        for (vertex, count) in &vertex_hex_count {
            assert!(*count >= 1 && *count <= 3, 
                "Vertex {:?} is touched by {} hexes (should be 1-3)", 
                vertex, count);
        }
        
        // Count how many vertices have each touch count
        let mut count_distribution: HashMap<usize, usize> = HashMap::new();
        for count in vertex_hex_count.values() {
            *count_distribution.entry(*count).or_insert(0) += 1;
        }
        
        println!("Vertex distribution: {:?}", count_distribution);
        // For a radius-2 hex board, we expect specific counts
    }

    #[test]
    fn test_each_edge_touches_two_hexes_max() {
        let board = Board::new();
        
        // Count how many hexes touch each edge
        let mut edge_hex_count: HashMap<EdgeCoord, usize> = HashMap::new();
        
        for (coord, _tile) in &board.tiles {
            for edge in coord.edges() {
                *edge_hex_count.entry(edge).or_insert(0) += 1;
            }
        }
        
        // Each edge should be touched by at most 2 hexes (border edges touch only 1)
        for (edge, count) in &edge_hex_count {
            assert!(*count >= 1 && *count <= 2, 
                "Edge {:?} is touched by {} hexes (should be 1-2)", 
                edge, count);
        }
        
        println!("Total edges: {}", edge_hex_count.len());
    }
}