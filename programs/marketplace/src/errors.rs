use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Math overflow")]
    MathOverFlow
}