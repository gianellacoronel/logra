use anchor_lang::prelude::*;

use crate::{
    constants::COMPLETION_SEED,
    error::LograError,
    state::{completion::Completion, mission::Mission},
};

#[derive(Accounts)]
pub struct CompleteMission<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,
    pub mission: Account<'info, Mission>,
    #[account(
        init,
        payer = participant,
        space = 8 + Completion::INIT_SPACE,
        seeds = [COMPLETION_SEED, mission.key().as_ref(), participant.key.as_ref()],
        bump
    )]
    pub completion: Account<'info, Completion>,
    pub system_program: Program<'info, System>,
}

pub fn complete_mission(ctx: Context<CompleteMission>) -> Result<()> {
    let completion = &mut ctx.accounts.completion;
    let mission = &mut ctx.accounts.mission;
    let current_time = Clock::get()?.unix_timestamp;

    require!(mission.is_active, LograError::MissionInactive);

    if let Some(start) = mission.start_at {
        require!(current_time > start, LograError::MissionNotStarted);
    }

    if let Some(end) = mission.end_at {
        require!(current_time < end, LograError::MissionExpired);
    }

    completion.mission = mission.key();
    completion.participant = ctx.accounts.participant.key();
    completion.completed_at = current_time;

    Ok(())
}
