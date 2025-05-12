use anchor_lang::prelude::*;

declare_id!("FXHsVskoXWrpMFVV8MVt1HEh2zJzTYZxbdcySDndCyQs");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello, world!"); // **** NEW LINE HERE ****

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
