use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::logo::LogoInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CoinExtendedInfo {
    /// A link to the logo, or a comment there is an on-chain logo stored
    pub logo: Option<LogoInfo>,
    /// A URL pointing to the project behind this token.
    pub project: Option<String>,
}