use crate::error::AdminError;
use crate::state::AdminConfig;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = admin @ AdminError::NotAuthorized,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    #[account(
        mut,
        seeds = [b"treasury", admin_config.key().as_ref()], 
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        // Transfer the fee amount to the admin's treasury account
        let cpi_program = self.system_program.to_account_info();

        let binding = self.admin_config.to_account_info().key();

        let cpi_accounts_fee = Transfer {
            from: self.treasury.to_account_info(),
            to: self.admin.to_account_info(),
        };

        let treasury_seeds = &[
            b"treasury",
            binding.as_ref(),
            &[self.admin_config.treasury_bump],
        ];

        let treasury_signer = &[&treasury_seeds[..]];

        let transfer_ctx_fee =
            CpiContext::new_with_signer(cpi_program, cpi_accounts_fee, treasury_signer);

        // Perform the transfer of the fee to the treasury
        transfer(transfer_ctx_fee, amount)
    }
}
