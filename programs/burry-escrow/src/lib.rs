use anchor_lang::prelude::*;
use instructions::{deposit::*, withdraw::*};

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("6d2fJan62kCY1BYxP7ZTKBxfzdjMkq8ncnCravQKwSM6");

#[program]
pub mod burry_escrow {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, escrow_amount: u64, unlock_price: f64) -> Result<()> {
        deposit_handler(ctx, escrow_amount, unlock_price)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }
}
