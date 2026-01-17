mod contexts;
mod error;
mod helpers;
mod states;

use anchor_lang::prelude::*;

use contexts::*;

declare_id!("FWQDYtsqrwKPc8hHLntfz7xavQbvuK2FNH7VtrbDeF5T");

#[program]
pub mod arrakis {
    use super::*;

    #[inline(never)]
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        name: String,
        token_yes_name: String,
        token_yes_symbol: String,
        token_no_name: String,
        token_no_symbol: String,
        token_yes_uri: String,
        token_no_uri: String,
        fee: u16,
        end_time: i64,
    ) -> Result<()> {
        ctx.accounts.save_market(
            seed,
            name,
            token_yes_name,
            token_yes_symbol,
            token_no_name,
            token_no_symbol,
            token_yes_uri,
            token_no_uri,
            fee,
            end_time,
            &ctx.bumps,
        )
    }

    pub fn add_liquidity(
        ctx: Context<Deposit>,
        max_yes: u64,
        max_no: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.deposit(max_no, max_yes, expiration)
    }
}
