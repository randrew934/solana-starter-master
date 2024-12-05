use crate::error::BetError;
use crate::state::{AdminConfig, List};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseGame<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // The signer (Judge or SBA Admin)

    #[account(
        mut,
        seeds = [b"list", list.maker.key().as_ref(), &list.bet_key.to_le_bytes()],
        bump = list.bump
    )]
    pub list: Account<'info, List>, // The game (list) being ended

    #[account(
        seeds = [b"admin_config"],
        bump = admin_config.bump
    )]
    pub admin_config: Account<'info, AdminConfig>, // Admin config holding the admin public key
}

impl<'info> CloseGame<'info> {
    pub fn close_game(&mut self) -> Result<()> {
        let list = &mut self.list; // Access the List account

        // Check if the game has either status 4 (ended but not closed) or 5 (appealed)
        if list.status != 4 && list.status != 5 {
            return Err(error!(BetError::InvalidGameStatus)); // Custom error if the game is not in a valid status to close
        }

        // Ensure the caller is authorized (Judge, Maker, or Admin)
        let caller = self.admin.key();

        if caller != self.admin_config.admin {
            return Err(error!(BetError::UnauthorizedAccess)); // Custom error
        }

        // Update the close timestamp to the current time
        list.close_timestamp = Clock::get()?.unix_timestamp as i128;

        // Calculate payout for each option
        let total_payout = list.pool_amount; // Total pool amount
        let total_bets = list.pool_no as u64; // Total number of bets placed

        // Ensure there are bets placed before calculating the payout
        if total_bets == 0 {
            return Err(error!(BetError::NoBetsPlaced)); // Custom error if no bets were placed
        }

        // Calculate the payout for the winning option
        let payout = if list.option_counts.len() > list.winner as usize {
            let winning_option_bets = list.option_counts[list.winner as usize];

            if winning_option_bets == 0 {
                return Err(error!(BetError::NoBetsPlaced)); // Custom error if no bets were placed on the winning option
            }

            // Calculate the payout: pool amount * amount per bet / number of bets on winning option

            let total_payout_sol = total_payout as f64 / 1_000_000_000.0; // Convert lamports to SOL

            let payout_per_bet_sol = (total_payout_sol) / (winning_option_bets as f64);

            // Convert the payout back to lamports after calculation
            let payout_per_bet_lamports = (payout_per_bet_sol * 1_000_000_000.0) as u64;

            payout_per_bet_lamports
        } else {
            return Err(error!(BetError::InvalidWinnerOption)); // Custom error if the winner option index is invalid
        };

        // Set the payout for the List state (payout for the winning option)
        list.payout = payout;

        // Optionally, update the status to "closed" (status 6) or any other status as needed
        list.status = 6; // Mark the game as closed

        Ok(())
    }
}
