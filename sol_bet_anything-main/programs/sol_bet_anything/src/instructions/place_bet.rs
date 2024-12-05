use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::error::BetError;
use crate::state::{Bet, List};

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // The user placing the bet

    #[account(
        mut,
        seeds = [b"list", list.maker.key().as_ref(), &list.bet_key.to_le_bytes()],
        bump = list.bump
    )]
    pub list: Account<'info, List>, // The game (list) the user is placing a bet on

    #[account(
        init_if_needed,
        payer = user,
        space = Bet::INIT_SPACE,
        seeds = [b"bet", user.key().as_ref(), &list.bet_key.to_le_bytes()],
        bump
    )]
    pub bet: Account<'info, Bet>, // The bet placed by the user

    #[account(
        mut,
        seeds = [b"list_treasury", list.key().as_ref()], 
        bump
    )]
    pub list_treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>, // System program to transfer funds
}

impl<'info> PlaceBet<'info> {
    pub fn send_sol(&mut self) -> Result<()> {
        let amount = self.list.amount;

        // Construct the CPI context for the transfer
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.list_treasury.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Perform the transfer
        transfer(cpi_ctx, amount)
    }

    pub fn place_bet(
        &mut self,
        option: u8, // Option selected by the user
        amount: u64,
        bumps: &PlaceBetBumps, // Amount of the bet
    ) -> Result<()> {
        let list = &mut self.list;
        let bet = &mut self.bet;
        let user = &self.user;

        // Check if the game has status 1(open)
        if list.status != 1 {
            return Err(error!(BetError::InvalidGameStatus));
        }

        // Ensure the betting period is valid
        let current_timestamp = Clock::get()?.unix_timestamp as i128;
        if current_timestamp > list.creation_timestamp + list.bet_period as i128 {
            return Err(error!(BetError::BetPeriodExpired)); // Custom error for expired bet period
        }

        // Ensure the user hasn't already placed a bet
        if bet.account != Pubkey::default() {
            return Err(error!(BetError::BetAlreadyPlaced)); // Custom error for already placed bet
        }

        // Place the bet by setting the Bet state
        bet.account = user.key(); // Set the user who placed the bet
        bet.bet_key = list.bet_key; // Associate the bet with the correct game (list)
        bet.option = option; // Store the selected option
        bet.bump = bumps.bet; // Store the bump for the bet PDA

        // Update the List state
        list.option_counts[option as usize] += 1;
        list.pool_amount += amount; // Add the amount to the pool
        list.pool_no += 1; // Increment the number of bets placed

        // Optionally: Implement fee deduction if required (e.g., transfer to the treasury)

        Ok(())
    }
}
