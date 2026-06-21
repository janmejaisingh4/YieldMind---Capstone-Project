use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::treasury::Treasury;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    if amount == 0 {
        return err!(ErrorCode::InvalidAllocation);
    }

    // transfer tokens from user -> treasury vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info().clone(),
        to: ctx.accounts.treasury_token_account.to_account_info().clone(),
        authority: ctx.accounts.user.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // update stats with checked math
    let treasury = &mut ctx.accounts.treasury;
    treasury.total_assets = treasury.total_assets.checked_add(amount).ok_or(ErrorCode::Overflow)?;
    treasury.total_deposits = treasury.total_deposits.checked_add(amount).ok_or(ErrorCode::Overflow)?;

    Ok(())
}
