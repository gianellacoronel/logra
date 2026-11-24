use anchor_lang::prelude::*;

use crate::{
    constants::{MISSION_COUNTER_SEED, MISSION_SEED}, state::{mission::Mission, mission_counter::MissionCounter},
};

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

pub fn create_mission(
    ctx: Context<CreateMission>,
    name: String,
    description: String,
    start_at: Option<i64>,
    end_at: Option<i64>
) -> Result<()> {
    let mission = &mut ctx.accounts.mission;
    let mission_counter = &mut ctx.accounts.mission_counter;

    if mission_counter.creator == Pubkey::default() {
        mission_counter.creator = ctx.accounts.creator.key();
        mission_counter.count = 0;
    } else {
        require_keys_eq!(ctx.accounts.creator.key(), mission_counter.creator);
    }

    mission.name = name;
    mission.description = description;
    mission.created_at = Clock::get()?.unix_timestamp;
    mission.start_at = start_at;
    mission.end_at = end_at;
    mission.creator = ctx.accounts.creator.key();
    mission.is_active = true;

    mission_counter.count += 1;

    Ok(())
}
