use crate::core::{player::player::Player};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Resource {
    Brick,
    Wood,
    Wool,
    Wheat,
    Ore,
    Uranium
}

pub fn spend_resources(player: &mut Player, cost: &[(Resource, u8)]) -> Result<(), String> {
    if cost.iter().any(|(res, amt)| player.playerhand.resources.get(res).unwrap_or(&0) < &amt) {
        return Err("Not enough resources".to_string());
    }

    for (res, amt) in cost {
        let entry = player.playerhand.resources.entry(*res).or_insert(0);
        *entry -= amt;
    }

    Ok(())
}