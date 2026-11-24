use anchor_lang::error_code;

#[error_code]
pub enum LograError {
    #[msg("The end date must be later than the start date.")]
    InvalidDateRange,
}
