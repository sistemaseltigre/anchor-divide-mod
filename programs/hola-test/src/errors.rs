use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
    #[msg("Invalid authority")]
    Unautorized,

    #[msg("Invalid authority account")]
    InvalidAuthorityAccount,
}