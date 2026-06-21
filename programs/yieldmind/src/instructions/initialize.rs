use anchor_lang::prelude::*;
use crate::state::treasury::Treasury;
use crate::constants::TREASURY_SEED;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [TREASURY_SEED, authority.key().as_ref()],
        bump,
        space = 8 + 200,
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeTreasury>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    if treasury.authority != Pubkey::default() {
        return err!(ErrorCode::TreasuryAlreadyInitialized);
    }

    treasury.authority = ctx.accounts.authority.key();
    treasury.treasury_bump = ctx.bumps.treasury;
    treasury.total_assets = 0;
    treasury.total_deposits = 0;
    treasury.total_withdrawals = 0;
    treasury.rebalance_count = 0;
    treasury.strategy_pubkey = Pubkey::default();
    treasury.created_at = Clock::get()?.unix_timestamp;

    Ok(())
}
