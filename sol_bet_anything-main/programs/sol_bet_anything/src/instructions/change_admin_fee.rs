use anchor_lang::prelude::*;

use crate::state::AdminConfig;

use crate::error::AdminError;

// Define the accounts structure for `change_admin`
#[derive(Accounts)]
pub struct ChangeAdminFee<'info> {
    #[account(mut)]
    // This is the AdminConfig account being updated
    pub admin_config: Account<'info, AdminConfig>,

    #[account(mut)]
    // The current admin signing the transaction
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>, // Reference to the System program (needed for signature verification)
}

impl<'info> ChangeAdminFee<'info> {
    pub fn change_admin_fee(&mut self, new_payout_fee: u8, new_appeal_fee: u64) -> Result<()> {
        // Check if the sender is the current admin
        if self.admin.key() != self.admin_config.admin {
            return Err(AdminError::NotAuthorized.into()); // Return the custom error
        }

        // Validate the new payout fee (it must be between 0 and 100)
        if new_payout_fee > 100 {
            return Err(AdminError::InvalidFee.into()); // Return error if fee is out of range
        }

        // Access the admin_config account and update the admin field
        let admin_config = &mut self.admin_config;
        admin_config.payout_fee = new_payout_fee;
        admin_config.appeal_fee = new_appeal_fee;

        msg!("Admin fee has been updated");

        Ok(())
    }
}
