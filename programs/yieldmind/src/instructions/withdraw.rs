use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::treasury::Treasury;
use crate::constants::TREASURY_SEED;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: authority must sign
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub treasury: Account<'info, Treasury>,

    #[account(mut, seeds = [TREASURY_SEED, authority.key().as_ref()], bump = treasury.treasury_bump)]
    /// CHECK: PDA signer
    pub treasury_signer: UncheckedAccount<'info>,

    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.authority {
        return err!(ErrorCode::Unauthorized);
    }

    if amount > treasury.total_assets {
        return err!(ErrorCode::InsufficientFunds);
    }

    let bump = treasury.treasury_bump;
    let bump_arr = [bump];
    let seeds = &[&[b"treasury", ctx.accounts.authority.key.as_ref(), &bump_arr[..]][..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info().clone(),
        to: ctx.accounts.destination_token_account.to_account_info().clone(),
        authority: ctx.accounts.treasury_signer.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
    token::transfer(cpi_ctx, amount)?;

    treasury.total_assets = treasury.total_assets.checked_sub(amount).ok_or(ErrorCode::Overflow)?;
    treasury.total_withdrawals = treasury.total_withdrawals.checked_add(amount).ok_or(ErrorCode::Overflow)?;

    Ok(())
}
