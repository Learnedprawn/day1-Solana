use anchor_lang::prelude::*;

declare_id!("8o3ehd3XnyDocd9hG1uz5trbmSRB7gaLaE9BCXDpEnMY");

#[program]
pub mod day {
    use super::*;

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        if a < 10 {
            return err!(MyError::AisTooSmall);
        }
        if a > 100 {
            return err!(MyError::AisTooBig);
        }
        msg!("Result changes = {}", a);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too big")]
    AisTooBig,
    #[msg("a is too small")]
    AisTooSmall,
}

// use anchor_lang::prelude::*;

// declare_id!("53hgft52DHUKMPHGu1kusuwxFGk2T8qngwSw2SyGRNrX");

// #[program]
// pub mod day {
//     use super::*;
//         // Import HashMap library
//     use std::collections::HashMap;

//     pub fn initialize(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
//         // Initialize the mapping
//         let mut my_map = HashMap::new();

//         // Add a key-value pair to the mapping
//         my_map.insert(key.to_string(), value.to_string());

//         // Log the value corresponding to a key from the mapping
//         msg!("My name is {}", my_map[&key]);

//         Ok(())
//     }
// }



