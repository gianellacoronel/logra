use anchor_lang::prelude::*;

use crate::{constants::{MISSION_COUNTER_SEED, MISSION_SEED}, state::{mission::Mission, mission_counter::MissionCounter}};

#[derive(Accounts)]
pub struct CreateMission<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init_if_needed,
        payer = creator,
        space = 8 + MissionCounter::INIT_SPACE,
        seeds = [MISSION_COUNTER_SEED, creator.key.as_ref()],
        bump
    )]
    pub mission_counter: Account<'info, MissionCounter>,
    #[account(
        init,
        payer = creator,
        space = 8 + Mission::INIT_SPACE,
        seeds = [MISSION_SEED, creator.key.as_ref(), mission_counter.count.to_le_bytes().as_ref()],
        bump
    )]
    pub mission: Account<'info, Mission>,
    pub system_program: Program<'info, System>,
}
