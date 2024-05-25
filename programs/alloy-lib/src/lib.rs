use anchor_lang::prelude::*;

declare_id!("5fTzHWbmUermD2tkncsj8gzHExhEfJUXM7AgAK9k65Mt");

#[program]
pub mod alloy_lib {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
