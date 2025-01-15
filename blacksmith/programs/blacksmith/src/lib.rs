use std::collections::HashSet;

use anchor_lang::prelude::*;
use anchor_lang::system_program;

use mpl_core::{programs::MPL_CORE_ID, Asset};

#[derive(Clone, Copy)]
enum ItemTypes {
    Chestplate,
    Gloves,
    Boots,
    Sword,
    Helmet,
    Neck,
    Ring,
}

impl ItemTypes {
    fn to_string(&self) -> String {
        match self {
            ItemTypes::Chestplate => "Chestplate".to_string(),
            ItemTypes::Gloves => "Gloves".to_string(),
            ItemTypes::Boots => "Boots".to_string(),
            ItemTypes::Sword => "Sword".to_string(),
            ItemTypes::Helmet => "Helmet".to_string(),
            ItemTypes::Neck => "Neck".to_string(),
            ItemTypes::Ring => "Ring".to_string(),
        }
    }

    fn random() -> Self {
        let variants = [
            ItemTypes::Chestplate,
            ItemTypes::Gloves,
            ItemTypes::Boots,
            ItemTypes::Sword,
            ItemTypes::Helmet,
            ItemTypes::Neck,
            ItemTypes::Ring,
        ];

        // seed for the random number is a combination of the slot_hash - timestamp
        let seed = match Clock::get() {
            Ok(clock_value) => clock_value.unix_timestamp,
            Err(_) => return ItemTypes::Chestplate,
        };

        let index: usize = match seed.checked_rem(variants.len().try_into().unwrap()) {
            Some(rem) => rem.try_into().unwrap(),
            None => return ItemTypes::Chestplate,
        };

        variants[index]
    }
}

declare_id!("4vbkSNKb9hx4DVe1md2CBzLwLwE8xsKAwBALe8CrNxVx");

#[program]
pub mod blacksmith {

    use std::borrow::Cow;

    use anchor_lang::{
        accounts::signer,
        solana_program::{self, system_program},
        system_program::{transfer, Transfer},
    };
    // use anchor_spl::metadata::mpl_token_metadata::instructions::{CreateV1CpiBuilder, UpdateAsAuthorityItemDelegateV2CpiBuilder};
    use mpl_core::instructions::{
        BurnCollectionV1CpiBuilder, BurnV1CpiBuilder, CreateCollectionV2CpiBuilder,
        CreateV2CpiBuilder, TransferV1CpiBuilder, UpdateV1CpiBuilder, UpdateV2CpiBuilder,
    };

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
                let cpi_ctx = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.signer.to_account_info(),
                        to: ctx.accounts.owner_pda.to_account_info(),
                    },
                );
                transfer(cpi_ctx, 10000)?; // TODO: Change this when deploying to prod to be 1000000000
            }
        } else {
            user.last_free_request_time = Some(now);
        }

        // Mint a new NFT for the user
        // let item_id = blacksmith::generate_random_item_id(now, &ctx.accounts.user.key());
        let generated_item = ItemTypes::random();

        let signer_seeds: &[&[&[u8]]] = &[&[b"owner_pda", &[ctx.bumps.owner_pda]]];

        let _ = CreateV2CpiBuilder::new(&ctx.accounts.core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .payer(&ctx.accounts.signer.to_account_info())
            .update_authority(Some(ctx.accounts.owner_pda.to_account_info()).as_ref())
            .owner(Some(ctx.accounts.signer.to_account_info()).as_ref())
            .authority(Some(ctx.accounts.owner_pda.to_account_info()).as_ref())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(generated_item.to_string())
            .uri(format!("https://raw.githubusercontent.com/PinkiePie1189/blockchain-project-contracts/refs/heads/master/jsons/{}.json?upgrade=0", generated_item.to_string().to_lowercase()).to_string())
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

    pub fn scrap_item(ctx: Context<ScrapItem>) -> Result<()> {
        let mut uri = "".to_string();
        {
            let metadata_account = &ctx.accounts.asset;

            // Assuming metadata is a standard Token Metadata Account (32 bytes + Metadata struct)
            let metadata_data = &metadata_account.data.borrow();

            // Parsing and verifying metadata...
            // Implement parsing logic here
            // Example: Deserialize it using the borsh crate or other methods

            msg!("Metadata raw data: {:?}", metadata_data);

            uri = Asset::from_bytes(&metadata_data).unwrap().base.uri;
        }

        let invokeResult = BurnV1CpiBuilder::new(&ctx.accounts.core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .authority(Some(ctx.accounts.payer.to_account_info()).as_ref())
            .payer(&ctx.accounts.payer.to_account_info())
            .invoke();

        // check if result is succcessful
        if !invokeResult.is_err() {
            // Parse the URL
            let parsed_url = parse_query_params(&uri);

            let mut upgradeValue = 0;

            for (key, value) in parsed_url {
                if key == "upgrade" {
                    match value.parse::<i32>() {
                        Ok(value) => {
                            upgradeValue = value;
                            break;
                        }
                        Err(_) => {
                            println!("The 'upgrade' parameter is not a valid integer.");
                        }
                    }
                }
            }

            let signer_seeds: &[&[&[u8]]] = &[&[b"owner_pda", &[ctx.bumps.authority]]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.authority.to_account_info(),
                    to: ctx.accounts.payer.to_account_info(),
                },
                signer_seeds
            );

            let transfer_amount = 1000 + upgradeValue * 1000;

            transfer(cpi_ctx, transfer_amount as u64)?;
        }
        Ok(())
    }

    pub fn upgrade_nft(ctx: Context<UpgradeNft>) -> Result<()> {
        let mut uri = "".to_string();
        {
            let metadata_account = &ctx.accounts.asset;

            // Assuming metadata is a standard Token Metadata Account (32 bytes + Metadata struct)
            let metadata_data = &metadata_account.data.borrow();

            // Parsing and verifying metadata...
            // Implement parsing logic here
            // Example: Deserialize it using the borsh crate or other methods

            msg!("Metadata raw data: {:?}", metadata_data);

            uri = Asset::from_bytes(&metadata_data).unwrap().base.uri;
        }

        // Parse the URL
        let parsed_url = parse_query_params(&uri);

        let mut incremented_value = 0;

        for (key, value) in parsed_url {
            if key == "upgrade" {
                match value.parse::<i32>() {
                    Ok(value) => {
                        incremented_value = std::cmp::min(value + 1, 9);
                        break;
                    }
                    Err(_) => {
                        println!("The 'upgrade' parameter is not a valid integer.");
                    }
                }
            }
        }

        let new_uri = update_query_param(&uri, "upgrade", incremented_value.to_string().as_str());

        let signer_seeds: &[&[&[u8]]] = &[&[b"owner_pda", &[ctx.bumps.authority]]];

        let _ = UpdateV2CpiBuilder::new(&ctx.accounts.core_program.to_account_info().clone())
            .system_program(&ctx.accounts.system_program.to_account_info().clone())
            .authority(Some(ctx.accounts.authority.to_account_info().clone()).as_ref())
            .asset(&ctx.accounts.asset.to_account_info().clone())
            .payer(&ctx.accounts.payer.to_account_info().clone())
            .new_uri(new_uri)
            .invoke_signed(signer_seeds);

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

fn parse_query_params(uri: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();

    // Split the URI into the base and query string
    if let Some(query_start) = uri.find('?') {
        let query_str = &uri[query_start + 1..];

        // Split the query string into key-value pairs
        for param in query_str.split('&') {
            let mut key_value = param.splitn(2, '=');
            let key = key_value.next().unwrap_or("").to_string();
            let value = key_value.next().unwrap_or("").to_string();
            params.push((key, value));
        }
    }

    params
}

fn update_query_param(uri: &str, param_name: &str, new_value: &str) -> String {
    // Split the URI into base and query string
    if let Some(query_start) = uri.find('?') {
        let base_uri = &uri[..query_start + 1];
        let query_str = &uri[query_start + 1..];

        // Rebuild the query string with updated parameter
        let mut found = false;
        let mut updated_query = query_str
            .split('&')
            .map(|pair| {
                let mut key_value = pair.splitn(2, '=');
                let key = key_value.next().unwrap_or("");
                let value = key_value.next().unwrap_or("");

                if key == param_name {
                    found = true;
                    format!("{}={}", key, new_value) // Update the specific parameter
                } else {
                    pair.to_string() // Keep other parameters unchanged
                }
            })
            .collect::<Vec<_>>();

        // If the parameter wasn't found, add it to the query string
        if !found {
            updated_query.push(format!("{}={}", param_name, new_value));
        }

        // Reconstruct the full URI
        format!("{}{}", base_uri, updated_query.join("&"))
    } else {
        // If there's no query string, just add the parameter
        format!("{}?{}={}", uri, param_name, new_value)
    }
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
    /// CHECK:` [MBLAC] Swift linter
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ScrapItem<'info> {
    /// CHECK:` [MBLAC] Swift linter
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// CHECK:` [MBLAC] Swift linter
    #[account(
        mut,
        seeds = [b"owner_pda"],
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    /// CHECK:` [MBLAC] Swift linter
    #[account(address = MPL_CORE_ID)]
    pub core_program: UncheckedAccount<'info>,

    /// CHECK:` [MBLAC] Swift linter
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:` [MBLAC] Swift linter
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpgradeNft<'info> {
    /// CHECK:` [MBLAC] Swift linter
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK:` [MBLAC] Swift linter
    pub core_program: UncheckedAccount<'info>,

    /// CHECK:` [MBLAC] Swift linter
    #[account(
        mut,
        seeds = [b"owner_pda"],
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    /// CHECK:` [MBLAC] Swift linter
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK:` [MBLAC] Swift linter
    pub system_program: AccountInfo<'info>,
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
    RandomFailed,
}

#[event]
pub struct ItemAssignedEvent {
    #[index]
    pub user: Pubkey,
    pub item_id: u64,
}
