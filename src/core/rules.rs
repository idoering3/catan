use crate::core::{player::player::{ Player }, resource::Resource};

// helper function
fn has_resources(player: &Player, cost: &[(Resource, u8)]) -> bool {
    cost.iter().all(|(res, amount)| player.get_resource_count(*res) >= *amount)
}

pub fn can_afford_road(player: &Player) -> bool {
    has_resources(player, &[
        (Resource::Brick, 1), 
        (Resource::Wood, 1)
    ])
}

pub fn can_afford_settlement(player: &Player) -> bool {
    has_resources(player, &[
        (Resource::Wheat, 1),
        (Resource::Brick, 1),
        (Resource::Wool, 1),
        (Resource::Wood, 1)
    ])
}

pub fn can_afford_city(player: &Player) -> bool {
    has_resources(player, &[
        (Resource::Wheat, 2), 
        (Resource::Ore, 3)
    ])
}

pub fn can_afford_development_card(player: &Player) -> bool {
    has_resources(player, &[
        (Resource::Wheat, 1), 
        (Resource::Ore, 1),
        (Resource::Wool, 1)
    ])
}

pub fn can_afford_icbm(player: &Player) -> bool {
    has_resources(player, &[
        (Resource::Uranium, 2),
        (Resource::Wool, 1)
    ])
}