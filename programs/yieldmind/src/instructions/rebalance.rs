use anchor_lang::prelude::*;
use crate::state::treasury::Treasury;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub treasury: Account<'info, Treasury>,
}

pub fn handler(ctx: Context<Rebalance>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.authority {
        return err!(ErrorCode::Unauthorized);
    }

    treasury.rebalance_count = treasury.rebalance_count.checked_add(1).ok_or(ErrorCode::Overflow)?;
    treasury.created_at = Clock::get()?.unix_timestamp;

    // simulate strategy update (no external integrations in MVP)

    Ok(())
}
