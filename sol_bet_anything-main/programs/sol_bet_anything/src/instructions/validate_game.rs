use crate::error::BetError;
use crate::state::List;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ValidateGame<'info> {
    #[account(mut)]
    pub judge: Signer<'info>, // Judge validating the game

    #[account(
        mut,
        seeds = [b"list", list.maker.key().as_ref(), &list.bet_key.to_le_bytes()],
        bump = list.bump,
        has_one = judge // Ensure this is the assigned judge for the list
    )]
    pub list: Account<'info, List>, // The game to validate
}

impl<'info> ValidateGame<'info> {
    pub fn validate_game(&mut self) -> Result<()> {
        let list = &mut self.list;

        // Ensure the game is in the correct status for validation
        if list.status != 0 {
            return Err(error!(BetError::InvalidGameStatus)); // Custom error if the game isn't active
        }

        // Update the game status to validated
        list.status = 1;

        Ok(())
    }
}
