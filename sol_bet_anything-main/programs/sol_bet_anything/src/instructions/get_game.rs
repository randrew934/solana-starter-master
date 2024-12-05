use anchor_lang::prelude::*;

use crate::state::List;

use crate::error::BetError;

#[derive(Accounts)]
pub struct GetGame<'info> {
    #[account()]
    pub list: Account<'info, List>, // Account to retrieve the game information from
}

impl<'info> GetGame<'info> {
    pub fn get_game(&mut self, bet_key: u64) -> Result<&List> {
        // Retrieve the game (ListState) by its bet_key
        let list = &self.list;

        // Ensure the bet_key matches the one in the ListState
        if list.bet_key != bet_key {
            return Err(error!(BetError::BetKeyMismatch)); // Custom error for mismatch
        }

        // Dereference the Account to get the List data and return it
        Ok(list) // Clone the List data from the Account
    }
}
