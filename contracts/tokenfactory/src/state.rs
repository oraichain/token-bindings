use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub fee: Option<Coin>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DENOM_OWNER: Map<String, Addr> = Map::new("denom_owner");
