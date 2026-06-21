use anchor_lang::prelude::*;
use crate::state::strategy::Strategy;
use crate::state::treasury::Treasury;
use crate::constants::STRATEGY_SEED;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateStrategy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init,
        payer = authority,
        seeds = [STRATEGY_SEED, treasury.key().as_ref()],
        bump,
        space = 8 + 200,
    )]
    pub strategy: Account<'info, Strategy>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateStrategy>, strategy_type: u8, risk_score: u8, allocation_percentage: u8) -> Result<()> {
    if allocation_percentage > 100 {
        return err!(ErrorCode::InvalidAllocation);
    }

    let strategy = &mut ctx.accounts.strategy;
    strategy.authority = ctx.accounts.authority.key();
    strategy.treasury = ctx.accounts.treasury.key();
    strategy.strategy_type = strategy_type;
    strategy.risk_score = risk_score;
    strategy.allocation_percentage = allocation_percentage;
    strategy.active = true;
    strategy.bump = ctx.bumps.strategy;

    // link to treasury
    ctx.accounts.treasury.strategy_pubkey = ctx.accounts.strategy.key();

    Ok(())
}
