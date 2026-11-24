use anchor_lang::error_code;

#[error_code]
pub enum MissionError {
    #[msg("The end date must be later than the start date.")]
    InvalidDateRange,
    #[msg("The mission name exceeds the maximum permitted length.")]
    MaxNameExceeded,
    #[msg("The mission description exceeds the maximum permitted length.")]
    MaxDescriptionExceeded,
    #[msg("The mission name is empty.")]
    NameEmpty,
    #[msg("The mission description is empty.")]
    DescriptionEmpty,
}
