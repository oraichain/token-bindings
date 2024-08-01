use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};
use token_bindings::Metadata;

#[cw_serde]
pub struct InstantiateMsg {
    pub fee: Option<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        fee: Option<Coin>,
    },
    CreateDenom {
        subdenom: String,
        metadata: Option<Metadata>,
    },
    ChangeDenomOwner {
        denom: String,
        new_admin_address: String,
    },
    ChangeAdmin {
        denom: String,
        new_admin_address: String,
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
    #[returns(token_bindings::MetadataResponse)]
    GetMetadata { denom: String },
    #[returns(token_bindings::DenomsByCreatorResponse)]
    DenomsByCreator { creator: String },
}
