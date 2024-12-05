use crate::error::BetError;
use crate::state::{AdminConfig, Appeal, List};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(bet_key: u64)]
pub struct MakeAppeal<'info> {
    #[account(
        mut,
        seeds = [b"list", list.maker.key().as_ref(), &bet_key.to_le_bytes()],
        bump = list.bump
    )]
    pub list: Account<'info, List>,

    #[account(
        init,
        payer = signer,
        seeds = [b"appeal", signer.key().as_ref(), &bet_key.to_le_bytes()],
        bump,
        space = Appeal::INIT_SPACE
    )]
    pub appeal: Account<'info, Appeal>,

    #[account(mut)]
    pub admin_config: Account<'info, AdminConfig>, // The Admin Config containing the Appeal fee

    #[account(
        mut,
        seeds = [b"treasury", admin_config.key().as_ref()], 
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> MakeAppeal<'info> {
    pub fn make_appeal(
        &mut self,
        bet_key: u64,
        description: String,
        appeal_url: String,
        bumps: &MakeAppealBumps,
    ) -> Result<()> {
        let list = &mut self.list;
        let appeal = &mut self.appeal;
        let user = &mut self.signer;

        // Ensure the game status allows appeals
        if list.status != 4 && list.status != 5 && list.status != 7 && list.status != 3 {
            return Err(error!(BetError::InvalidGameStatus));
        }

        // Update the Appeal account
        appeal.account = user.key();
        appeal.bet_key = bet_key;
        appeal.description = description;
        appeal.appeal_url = appeal_url;
        appeal.bump = bumps.appeal;

        // Update the List state to indicate an appeal
        list.appealed = 1;
        list.status = 3;

        Ok(())
    }

    pub fn send_sol(&mut self) -> Result<()> {
        let admin_config = &self.admin_config;

        let appeal_fee = admin_config.appeal_fee as u64;

        // Ensure the user has enough funds to pay the appeal fee
        if self.signer.lamports() < appeal_fee {
            return Err(error!(BetError::InsufficientFunds)); // Custom error if the user doesn't have enough SOL
        }

        // Prepare the CPI transfer to move the appeal fee to the treasury
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(), // User pays the fee
            to: self.treasury.to_account_info(), // Treasury account
        };

        let transfer_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Perform the transfer of the appeal fee
        transfer(transfer_ctx, appeal_fee)?;

        Ok(())
    }
}
