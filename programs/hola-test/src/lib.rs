//declare_id!("9yYQx48mQ2AR54yCVhuPkePSghykpw87ZJCJWHkwthqF");
use anchor_lang::prelude::*;

mod instructions;
mod nft_accounts;
pub mod errors;

use crate::nft_accounts::single_accounts::SingleNFT;
use crate::instructions::single_nft::create_single_nft;  

declare_id!("9yYQx48mQ2AR54yCVhuPkePSghykpw87ZJCJWHkwthqF");

#[program]
pub mod nft_program {
    use super::*;
    pub fn create_nft(
        ctx: Context<SingleNFT>,
        id: String,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        create_single_nft(ctx, id, name, symbol, uri)
    }
}