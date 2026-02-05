use crate::core::{board::HexCoord, resource::Resource};

// we want to define our tile now. It has a hex coordinate, but also will have several different placement positions on the tile. 

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tile {
    pub resource: Option<Vec<Resource>>,
    pub tile_type: Option<TileResource>,
    pub dice_number: Option<u8>,
    pub coord: HexCoord
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileResource {
    Forest,
    Hill,
    Pasture,
    Field,
    Mountain,
    Desert,
    Swamp
}
