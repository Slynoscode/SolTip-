use anchor_lang::prelude::*;

#[account]
pub struct Soltip {
    pub authority: Pubkey,
    pub tip_amount: u64,
    pub tip_recipient: Pubkey,
    pub tip_timestamp: i64,
}