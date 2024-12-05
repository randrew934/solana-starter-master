use anchor_lang::prelude::*;

use crate::state::AdminConfig;

use crate::error::AdminError;

// Define the accounts structure for `change_admin`
#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    #[account(mut)]
    // This is the AdminConfig account being updated
    pub admin_config: Account<'info, AdminConfig>,

    #[account(mut)]
    // The current admin signing the transaction
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>, // Reference to the System program (needed for signature verification)
}

impl<'info> ChangeAdmin<'info> {
    pub fn change_admin(&mut self, new_admin: Pubkey) -> Result<()> {
        // Check if the sender is the current admin
        if self.admin.key() != self.admin_config.admin {
            return Err(AdminError::NotAuthorized.into()); // Return the custom error
        }

        // Access the admin_config account and update the admin field
        let admin_config = &mut self.admin_config;
        admin_config.admin = new_admin;
        Ok(())
    }
}
