use anchor_lang::prelude::*;

use crate::{
    constants::COMPLETION_SEED,
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
