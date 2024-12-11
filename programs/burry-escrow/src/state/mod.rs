use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub unlock_price: f64,  // 允许提款的 SOL 美元价格
    pub escrow_amount: u64, // 托管账户中持有的 lamports 数量
}
