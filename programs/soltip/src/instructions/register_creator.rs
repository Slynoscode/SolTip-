use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct RegisterCreator<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 200,
        seeds = [b"creator", 
        authority.key().as_ref()],
        bump,
    )]
    pub creator: Account<'info, Soltip>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_register_creator(ctx: Context<RegisterCreator>, _handle: String) -> Result<()> {
    let creator = &mut ctx.accounts.creator;
    creator.authority = ctx.accounts.authority.key();
    creator.tip_amount = 0;
    creator.tip_recipient = ctx.accounts.authority.key();
    creator.tip_timestamp = Clock::get()?.unix_timestamp;
    Ok(())
}