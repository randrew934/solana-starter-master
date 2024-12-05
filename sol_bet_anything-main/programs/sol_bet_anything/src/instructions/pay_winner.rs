use crate::error::BetError;
use crate::state::{AdminConfig, Bet, List};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct PayWinner<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // User who is receiving the payout

    #[account(mut)]
    pub list: Account<'info, List>, // The game list containing the options and pool

    #[account(mut)]
    pub admin_config: Account<'info, AdminConfig>, // The Admin Config containing the payout fee

    #[account(mut)] // Bet account for the user, which will be closed after payout
    pub bet: Account<'info, Bet>, // User's bet placed in the game

    #[account(
        mut,
        seeds = [b"treasury", admin_config.key().as_ref()], 
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"list_treasury", list.key().as_ref()], 
        bump
    )]
    pub list_treasury: SystemAccount<'info>,

    // System program to handle transfers
    pub system_program: Program<'info, System>,
}

impl<'info> PayWinner<'info> {
    pub fn pay_winner(&mut self) -> Result<()> {
        let list = &mut self.list;
        let bet = &mut self.bet;
        let user = &mut self.user;
        let admin_config = &mut self.admin_config;

        // Ensure the game has ended and the winner has been declared
        if list.status != 6 {
            return Err(error!(BetError::GameNotEnded)); // Custom error if the game hasn't ended
        }

        // Ensure the user selected the winning option
        if bet.option != list.winner {
            return Err(error!(BetError::NotAWinner)); // Custom error if the user didn't select the winning option
        }

        // Get the payout amount from the Bet account
        let mut payout = list.payout;

        // Get the payout fee from AdminConfig (as a percentage)
        let payout_fee = admin_config.payout_fee as u64;

        // Calculate the fee amount and subtract it from the payout
        let fee_amount = (payout * payout_fee) / 100; // Payout fee is a percentage of the payout
        payout = payout.checked_sub(fee_amount).unwrap_or(0); // Ensure payout doesn't go negative

        // Ensure there are enough funds in the List PDA to pay the winner
        if list.pool_amount < payout {
            return Err(error!(BetError::InsufficientFunds)); // Custom error if the List PDA doesn't have enough funds
        }

        // Transfer the payout from the List PDA to the user's account
        let cpi_program = self.system_program.to_account_info();
        let binding = self.list.to_account_info().key();
        let cpi_accounts = Transfer {
            from: self.list_treasury.to_account_info(),
            to: user.to_account_info(),
        };
        let seeds = &[
            b"list_treasury",
            binding.as_ref(),
            &[self.list.treasury_bump],
        ];
        let pda_signer = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, pda_signer);

        // Perform the transfer
        transfer(transfer_ctx, payout)?;

        // Transfer the fee amount to the admin's treasury account
        let treasury_account = self.treasury.to_account_info();
        let cpi_program = self.system_program.to_account_info();
        let binding = self.list.to_account_info().key();

        let cpi_accounts_fee = Transfer {
            from: self.list_treasury.to_account_info(),
            to: treasury_account,
        };
        let treasury_seeds = &[
            b"list_treasury",
            binding.as_ref(),
            &[self.list.treasury_bump],
        ];
        let treasury_signer = &[&treasury_seeds[..]];

        let transfer_ctx_fee =
            CpiContext::new_with_signer(cpi_program, cpi_accounts_fee, treasury_signer);

        // Perform the transfer of the fee to the treasury
        transfer(transfer_ctx_fee, fee_amount)?;

        // After the transfer, we can close the Bet account
        let bet_lamports = **bet.to_account_info().lamports.borrow();
        **self.user.to_account_info().lamports.borrow_mut() += bet_lamports; // Transfer lamports to user
        **bet.to_account_info().lamports.borrow_mut() = 0; // Set bet account balance to zero
        bet.to_account_info().try_borrow_mut_data()?.fill(0);

        Ok(())
    }
}
