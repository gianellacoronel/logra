#[account]
#[derive(InitSpace)]
pub struct Mission {
    pub creator: Pubkey,
    #[max_len(100)]
    pub name: String,
    #[max_len(500)]
    pub description: String,
    pub created_at: i64,
    pub start_at: Option<i64>,
    pub end_at: Option<i64>,
    pub is_active: bool,
}
