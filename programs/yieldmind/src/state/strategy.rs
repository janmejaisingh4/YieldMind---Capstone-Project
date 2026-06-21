use anchor_lang::prelude::*;

#[account]
pub struct Strategy {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub strategy_type: u8,
    pub risk_score: u8,
    pub allocation_percentage: u8,
    pub active: bool,
    pub bump: u8,
}

impl Strategy {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 1 + 1 + 1 + 1;
}
