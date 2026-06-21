use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub authority: Pubkey,
    pub treasury_bump: u8,
    pub total_assets: u64,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub rebalance_count: u64,
    pub strategy_pubkey: Pubkey,
    pub created_at: i64,
}

impl Treasury {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 8 + 8 + 8 + 32 + 8; // discriminator + fields
}
