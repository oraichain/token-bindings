use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use token_bindings::Metadata;

use crate::info::CoinExtendedInfo;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
    },
    CreateDenom {
        subdenom: String,
        metadata: Option<Metadata>,
        extended_info: Option<CoinExtendedInfo>
    },
    ChangeDenomOwner {
        denom: String,
        new_admin_address: String,
    },
    ChangeAdmin {
        denom: String,
        new_admin_address: String,
    },
    UpdateExtendedInfo {
        denom: String,
        extended_info: CoinExtendedInfo,
    },
    MintTokens {
        denom: String,
        amount: Uint128,
        mint_to_address: String,
    },
    BurnTokens {
        denom: String,
        amount: Uint128,
        burn_from_address: String,
    },
    ForceTransfer {
        denom: String,
        amount: Uint128,
        from_address: String,
        to_address: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(token_bindings::FullDenomResponse)]
    GetDenom {
        creator_address: String,
        subdenom: String,
    },
    #[returns(MetadataResponse)]
    GetMetadata { denom: String },
    #[returns(token_bindings::DenomsByCreatorResponse)]
    DenomsByCreator { creator: String },
    #[returns(token_bindings::ParamsResponse)]
    GetParams {},
}

#[cw_serde]
pub struct MetadataResponse {
    pub metadata: Option<Metadata>,
    pub extended_info: Option<CoinExtendedInfo>,
}
