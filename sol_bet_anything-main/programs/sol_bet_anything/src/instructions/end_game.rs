use crate::error::BetError;
use crate::state::{AdminConfig, List};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EndGame<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // The caller (Judge or SBA Admin)

    #[account(
        mut,
        seeds = [b"list", list.maker.key().as_ref(), &list.bet_key.to_le_bytes()],
        bump = list.bump,
        constraint = list.status == 1 @ BetError::InvalidGameStatus // Game must be active (status == 1)
    )]
    pub list: Account<'info, List>, // The game (list) being ended

    #[account(
        seeds = [b"admin_config"],
        bump = admin_config.bump
    )]
    pub admin_config: Account<'info, AdminConfig>, // Admin config holding the admin public key
}

impl<'info> EndGame<'info> {
    pub fn end_game(&mut self) -> Result<()> {
        let list = &mut self.list;
        let admin_config = &self.admin_config;

        // Ensure the caller is authorized (Judge, Maker, or Admin)
        let caller = self.signer.key();

        if caller != list.judge && caller != admin_config.admin {
            return Err(error!(BetError::UnauthorizedAccess)); // Custom error
        }

        // Ensure the game has reached the end of its betting period
        let current_timestamp = Clock::get()?.unix_timestamp as i128;

        if current_timestamp < list.creation_timestamp + list.bet_period as i128 {
            return Err(error!(BetError::GameStillActive)); // Custom error
        }

        // Set the status based on who called the instruction
        if caller == admin_config.admin {
            list.status = 7; // Admin ends the game
        } else if caller == list.judge {
            list.status = 2; // Judge ends the game
        } else {
            return Err(error!(BetError::UnauthorizedAccess)); // If not admin or judge
        }

        list.ended_timestamp = current_timestamp as i128;

        Ok(())
    }
}
