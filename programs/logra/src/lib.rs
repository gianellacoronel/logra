pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9LuaFGGGnZuLENWNywVR5fppfGuH7zYFPY33gmnDhXbK");

#[program]
pub mod logra {
    use super::*;

    pub fn create_mission(
        ctx: Context<CreateMission>,
        name: String,
        description: String,
        start_at: Option<i64>,
        end_at: Option<i64>,
    ) -> Result<()> {
        crate::instructions::create_mission::create_mission(
            ctx,
            name,
            description,
            start_at,
            end_at,
        )
    }

    pub fn complete_mission(ctx: Context<CompleteMission>) -> Result<()> {
        crate::instructions::complete_mission::complete_mission(ctx)
    }
}
