#[account]
#[derive(InitSpace)]
pub struct Completion {
    pub mission: Pubkey,
    pub participant: Pubkey,
    pub completed_at: i64,
}
