use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::{
    query::{FullDenomResponse, TokenFactoryQuery, TokenFactoryQueryEnum},
    DenomsByCreatorResponse, MetadataResponse,
};

/// This is a helper wrapper to easily use our custom queries
pub struct TokenQuerier<'a> {
    querier: &'a QuerierWrapper<'a, TokenFactoryQuery>,
}

impl<'a> TokenQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<TokenFactoryQuery>) -> Self {
        TokenQuerier { querier }
    }

    pub fn full_denom(
        &self,
        creator_addr: String,
        subdenom: String,
    ) -> StdResult<FullDenomResponse> {
        let full_denom_query = TokenFactoryQuery::Token(TokenFactoryQueryEnum::FullDenom {
            creator_addr,
            subdenom,
        });
        self.querier.query(&full_denom_query.into())
    }

    pub fn denom_by_creator(&self, creator: String) -> StdResult<DenomsByCreatorResponse> {
        let denom_by_creator_query =
            TokenFactoryQuery::Token(TokenFactoryQueryEnum::DenomsByCreator { creator });
        self.querier.query(&denom_by_creator_query.into())
    }

    pub fn metadata(&self, denom: String) -> StdResult<MetadataResponse> {
        let metadata_query = TokenFactoryQuery::Token(TokenFactoryQueryEnum::Metadata { denom });
        self.querier.query(&metadata_query.into())
    }
}
