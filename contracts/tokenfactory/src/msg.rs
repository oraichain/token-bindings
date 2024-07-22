use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use token_bindings::{DenomsByCreatorResponse, FullDenomResponse, Metadata, MetadataResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub fee_denom: String,
    pub fee_amount: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
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
    #[returns(FullDenomResponse)]
    GetDenom {
        creator_address: String,
        subdenom: String,
    },
    #[returns(MetadataResponse)]
    GetMetadata { denom: String },
    #[returns(DenomsByCreatorResponse)]
    DenomsByCreator { creator: String },
}
