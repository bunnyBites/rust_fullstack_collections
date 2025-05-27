use anchor_lang::prelude::*;

declare_id!("CzE7YZjeeZn22ST6pD4Zo2XYWpNHgD7EVti9RWbN5dU6");

#[program]
pub mod blockchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
