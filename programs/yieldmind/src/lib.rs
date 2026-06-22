#![allow(unexpected_cfgs)]
#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use crate::instructions::initialize;
use crate::instructions::deposit;
use crate::instructions::withdraw;
use crate::instructions::strategy;
use crate::instructions::rebalance;

declare_id!("Ch4GZssLa7sXnS1cWTwatM2GtJu3SbQ6bSTgkz4EsuJ6");

#[program]
pub mod yieldmind {
    use super::*;

    pub fn initialize_treasury(ctx: Context<initialize::InitializeTreasury>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn deposit(ctx: Context<deposit::Deposit>, amount: u64) -> Result<()> {
        deposit::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<withdraw::Withdraw>, amount: u64) -> Result<()> {
        withdraw::handler(ctx, amount)
    }

    pub fn create_strategy(ctx: Context<strategy::CreateStrategy>, strategy_type: u8, risk_score: u8, allocation_percentage: u8) -> Result<()> {
        strategy::handler(ctx, strategy_type, risk_score, allocation_percentage)
    }

    pub fn rebalance(ctx: Context<rebalance::Rebalance>) -> Result<()> {
        rebalance::handler(ctx)
    }
}