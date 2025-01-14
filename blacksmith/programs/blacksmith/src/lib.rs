use std::collections::HashSet;

use anchor_lang::prelude::*;
use anchor_lang::system_program;


use anchor_spl::{
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, mpl_token_metadata::types::DataV2
    },
    token::{Token, TokenAccount, MintTo, mint_to},
};
use mpl_core::{programs::MPL_CORE_ID, Asset};
use core::mem::size_of;



declare_id!("4vbkSNKb9hx4DVe1md2CBzLwLwE8xsKAwBALe8CrNxVx");

#[program]
pub mod blacksmith {

    use anchor_lang::{accounts::signer, solana_program::system_program, system_program::Transfer, system_program::transfer};
    use anchor_spl::metadata::mpl_token_metadata::instructions::CreateV1CpiBuilder;
    use mpl_core::instructions::{CreateCollectionV2CpiBuilder, CreateV2CpiBuilder, TransferV1CpiBuilder};

    use super::*;

    pub fn request_item(ctx: Context<RequestItem>, pay_with_token: bool) -> Result<()> {
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;

        let user = &mut ctx.accounts.user;

        // // Determine if this is a free request or requires payment
        if let Some(last_request) = user.last_free_request_time {
            if now >= last_request + 24 * 60 * 60 {
                user.last_free_request_time = Some(now);
            } else {
                // If pay_with_token is false and free item unavailable, throw an error
                require!(pay_with_token, CustomError::FreeItemUnavailable);
                // Deduct tokens for paid item
                let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.owner_pda.to_account_info()
                });
                transfer(cpi_ctx, 1000000000)?;
            }
        } else {
            user.last_free_request_time = Some(now);
        }

        // Mint a new NFT for the user
        // let item_id = blacksmith::generate_random_item_id(now, &ctx.accounts.user.key());

        let signer_seeds: &[&[&[u8]]] = &[&[b"owner_pda", &[ctx.bumps.owner_pda]]];

        let _ = CreateV2CpiBuilder::new(&ctx.accounts.core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .payer(&ctx.accounts.signer.to_account_info())
            .update_authority(Some(ctx.accounts.owner_pda.to_account_info()).as_ref())
            .owner(Some(ctx.accounts.signer.to_account_info()).as_ref())
            .authority(Some(ctx.accounts.owner_pda.to_account_info()).as_ref())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name("Sword + 9".to_string())
            .uri("http://example.com".to_string())
            .invoke_signed(signer_seeds);
    
            // mint_to(cpi_context, 1)?;
    
            // // create metadata account
            // let cpi_context = CpiContext::new(
            //     ctx.accounts.token_metadata_program.to_account_info(),
            //     CreateMetadataAccountsV3 {
            //         metadata: ctx.accounts.metadata_account.to_account_info(),
            //         mint: ctx.accounts.mint_account.to_account_info(),
            //         mint_authority: ctx.accounts.signer.to_account_info(),
            //         update_authority: ctx.accounts.signer.to_account_info(),
            //         payer: ctx.accounts.signer.to_account_info(),
            //         system_program: ctx.accounts.system_program.to_account_info(),
            //         rent: ctx.accounts.rent.to_account_info(),
            //     },
            // );
    
            // let data_v2 = DataV2 {
            //     name: "Sword".to_string(),
            //     symbol: "SWO".to_string(),
            //     uri: "http://example.com".to_string(),
            //     seller_fee_basis_points: 0,
            //     creators: None,
            //     collection: None,
            //     uses: None,
            // };
    
            // create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;
    
            // //create master edition account
            // let cpi_context = CpiContext::new(
            //     ctx.accounts.token_metadata_program.to_account_info(),
            //     CreateMasterEditionV3 {
            //         edition: ctx.accounts.master_edition_account.to_account_info(),
            //         mint: ctx.accounts.mint_account.to_account_info(),
            //         update_authority: ctx.accounts.signer.to_account_info(),
            //         mint_authority: ctx.accounts.signer.to_account_info(),
            //         payer: ctx.accounts.signer.to_account_info(),
            //         metadata: ctx.accounts.metadata_account.to_account_info(),
            //         token_program: ctx.accounts.token_program.to_account_info(),
            //         system_program: ctx.accounts.system_program.to_account_info(),
            //         rent: ctx.accounts.rent.to_account_info(),
            //     },
            // );
    
            // create_master_edition_v3(cpi_context, None)?;

        Ok(())
    }

    pub fn transfer_nft(ctx: Context<TransferNft>) -> Result<()> {

        TransferV1CpiBuilder::new(&ctx.accounts.core_program.to_account_info())
        .asset(&ctx.accounts.asset)
        .payer(&ctx.accounts.signer)
        .new_owner(&ctx.accounts.new_owner)
        .invoke()?;
        Ok(())
    }

    // fn mint_nft(
    //     mint_account: AccountInfo,
    //     associated_token_account: AccountInfo,
    //     signer: &Signer,
    //     user_data: &AccountInfo,
    //     metadata_account: &AccountInfo,
    //     master_edition_account: &AccountInfo,
    //     token_program: &AccountInfo,
    //     token_metadata_program: &AccountInfo,
    //     system_program: &AccountInfo,
    //     rent: &Sysvar<Rent>,
    //     user: Pubkey,
    //     item_id: u64,
    // ) -> Result<()> {
       


    //     Ok(())
    // }
}


// fn generate_random_item_id(timestamp: i64, user_key: Pubkey) -> u64 {
//     let seed = [timestamp.to_be_bytes(), user_key.to_bytes()].concat();
//     let hash = anchor_lang::solana_program::hash::hash(&seed);
//     u64::from_be_bytes(hash.to_bytes()[..8].try_into().unwrap())
// }

#[derive(Accounts)]
pub struct TransferNft<'info> {

    // #[account(mut)]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    /// CHECK:` doc comment explaining why no checks through types are necessary
    pub asset: AccountInfo<'info>,
    #[account(mut)]
     /// CHECK:` [MBLAC] Swift linter
    pub new_owner: AccountInfo<'info>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK:` [MBLAC] Swift linter
    pub core_program: UncheckedAccount<'info>,
   
}

#[derive(Accounts)]
pub struct RequestItem<'info> {

    #[account(
        mut,
        // payer = signer,
        // space = 8 + size_of::<User>(),
        seeds = [b"user_pda"],
        bump,
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub signer: Signer<'info>,


    #[account(
        mut,
        seeds = [b"owner_pda"],
        bump,
    )]
    /// CHECK:` [MBLAC] Swift linter
    pub owner_pda: UncheckedAccount<'info>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK:` [MBLAC] Swift linter
    pub core_program: UncheckedAccount<'info>,

    #[account(mut)]
    pub asset: Signer<'info>,
    pub system_program: Program<'info, System>,
   
}



#[account]
pub struct User {
    pub last_free_request_time: Option<i64>,
    pub payment_amount: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("You must wait 24 hours to request another free item.")]
    RequestNotAllowedYet,
    #[msg("Duplicate item ID detected.")]
    DuplicateItemId,
    #[msg("Free item unavailable. Please pay with tokens.")]
    FreeItemUnavailable,
}

#[event]
pub struct ItemAssignedEvent {
    #[index]
    pub user: Pubkey,
    pub item_id: u64,
}

