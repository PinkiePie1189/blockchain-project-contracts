declare_id!("HjwkPBteru9h52PN6zy4fqfMiRLKMrFoyhgtM5YmkPnA");

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

#[program]
pub mod auction_house {
    use super::*;

    pub fn initialize_auction(
        ctx: Context<InitializeAuction>,
        price: u64,
        duration: i64,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.seller = *ctx.accounts.seller.key;
        auction.nft_mint = ctx.accounts.nft_mint.key();
        auction.price = price;
        auction.expiry_time = Clock::get()?.unix_timestamp + duration;

        // Transfer NFT to the program's NFT account
        token::transfer(ctx.accounts.into_transfer_to_program_context(), 1)?;

        Ok(())
    }

    pub fn buy_item(ctx: Context<BuyItem>) -> Result<()> {
        let auction = &ctx.accounts.auction;

        // Check expiry
        require!(
            Clock::get()?.unix_timestamp <= auction.expiry_time,
            AuctionError::AuctionExpired
        );

        // Transfer funds to the seller
        **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += auction.price;
        **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? -= auction.price;

        // Transfer NFT to buyer
        token::transfer(ctx.accounts.into_transfer_to_buyer_context(), 1)?;

        // Close the auction account
        auction.close(ctx.accounts.seller.to_account_info())?;

        Ok(())
    }

    pub fn claim_unsold_item(ctx: Context<ReturnUnsoldItem>) -> Result<()> {
        let auction = &ctx.accounts.auction;

        // TODO: maybe remove this check
        // Validate that the auction has expired
        require!(
            Clock::get()?.unix_timestamp > auction.expiry_time,
            AuctionError::AuctionStillActive
        );

        // Transfer NFT back to the seller
        token::transfer(ctx.accounts.into_transfer_to_seller_context(), 1)?;

        // Close the auction account to clean up storage
        auction.close(ctx.accounts.seller.to_account_info())?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAuction<'info> {
    #[account(init, payer = seller, space = 8 + AuctionListing::LEN)]
    pub auction: Account<'info, AuctionListing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub seller_nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_nft_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeAuction<'info> {
    pub fn into_transfer_to_program_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.seller_nft_account.to_account_info(),
            to: self.program_nft_account.to_account_info(),
            authority: self.seller.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct BuyItem<'info> {
    #[account(mut, has_one = seller)]
    pub auction: Account<'info, AuctionListing>,
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_nft_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> BuyItem<'info> {
    pub fn into_transfer_to_buyer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.program_nft_account.to_account_info(),
            to: self.buyer_nft_account.to_account_info(),
            authority: self.program_nft_account.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct ReturnUnsoldItem<'info> {
    #[account(mut, has_one = seller)]
    pub auction: Account<'info, AuctionListing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub program_nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_nft_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ReturnUnsoldItem<'info> {
    pub fn into_transfer_to_seller_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.program_nft_account.to_account_info(),
            to: self.seller_nft_account.to_account_info(),
            authority: self.program_nft_account.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

#[account]
pub struct AuctionListing {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
    pub expiry_time: i64,
}

impl AuctionListing {
    pub const LEN: usize = 32 + 32 + 8 + 8;
}

#[error_code]
pub enum AuctionError {
    #[msg("The auction has expired.")]
    AuctionExpired,
    #[msg("The auction is still active.")]
    AuctionStillActive,
}
