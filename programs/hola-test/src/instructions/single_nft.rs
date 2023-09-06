use anchor_lang::prelude::*;
use mpl_token_metadata::state::{DataV2};
use anchor_spl::token::{mint_to, MintTo};
use anchor_spl::metadata::{
    create_master_edition_v3,
    create_metadata_accounts_v3,
    CreateMasterEditionV3,
    CreateMetadataAccountsV3
};

use crate::nft_accounts::single_accounts::SingleNFT;

pub fn create_single_nft(
    ctx: Context<SingleNFT>,
    id: String,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    //  "mint".as_bytes().as_ref()  
    msg!("Creating seeds");

   
    let seeds = &[
        "mint".as_bytes().as_ref(),
        id.as_bytes().as_ref(),
        &[*ctx.bumps.get("mint").unwrap()],
    ];

    msg!("Run mint_to");

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &[&seeds[..]],
        ),
        1, // 1 token
    )?;

    msg!("Run create metadata accounts v3");

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[&seeds[..]],
        ),
        DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    msg!("Run create master edition v3");

    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[&seeds[..]],
        ),
        Some(1),
    )?;

    msg!("Minted NFT successfully");

    Ok(())
}