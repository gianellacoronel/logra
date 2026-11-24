use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MissionCounter {
    pub creator: Pubkey,
    pub count: u64,
}
