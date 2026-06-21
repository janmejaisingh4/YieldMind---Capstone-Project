use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized action")]
    Unauthorized,

    #[msg("Treasury already initialized")]
    TreasuryAlreadyInitialized,

    #[msg("Invalid strategy")]
    InvalidStrategy,

    #[msg("Invalid allocation percentage")]
    InvalidAllocation,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Overflow")]
    Overflow,
}
