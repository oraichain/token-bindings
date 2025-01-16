use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, Default)]
pub struct Creator {
    pub whitelist_addresses: Vec<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const CREATOR: Item<Creator> = Item::new("creator");
pub const DENOM_OWNER: Map<String, Addr> = Map::new("denom_owner");
