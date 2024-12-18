use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ExtendedInfo {
    pub uri: Option<String>,
    pub uri_hash: Option<String>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DENOM_OWNER: Map<String, Addr> = Map::new("denom_owner");
pub const DENOM_EXTENDED_INFO: Map<String, ExtendedInfo> = Map::new("denom_extended_info");
