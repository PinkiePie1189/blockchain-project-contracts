use anchor_lang::prelude::*;

declare_id!("HjwkPBteru9h52PN6zy4fqfMiRLKMrFoyhgtM5YmkPnA");

#[program]
pub mod auction_house {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
