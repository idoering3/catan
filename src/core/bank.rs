use std::collections::HashMap;

use crate::core::resource::Resource;


// the bank will have a set amount of resources at the beginning of the game.
pub struct Bank {
    pub resources: HashMap<Resource, u8>
}