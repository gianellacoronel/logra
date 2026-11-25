use anchor_lang::error_code;

#[error_code]
pub enum LograError {
    #[msg("The end date must be later than the start date.")]
    MissionInvalidDateRange,
    #[msg("The mission name exceeds the maximum permitted length.")]
    MissionMaxNameExceeded,
    #[msg("The mission description exceeds the maximum permitted length.")]
    MissionMaxDescriptionExceeded,
    #[msg("The mission name is empty.")]
    MissionNameEmpty,
    #[msg("The mission description is empty.")]
    MissionDescriptionEmpty,
    #[msg("The mission is not active.")]
    MissionInactive,
    #[msg("The mission has not started yet.")]
    MissionNotStarted,
    #[msg("The mission has already expired.")]
    MissionExpired,
}
