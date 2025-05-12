use anchor_lang::prelude::*;

declare_id!("FXHsVskoXWrpMFVV8MVt1HEh2zJzTYZxbdcySDndCyQs");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
