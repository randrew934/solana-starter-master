use crate::error::AppealError;
use crate::state::{AdminConfig, Appeal};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bet_account: Pubkey, bet_key: u64)]
pub struct GetAppeal<'info> {
    /// Signer must match the admin in `AdminConfig`
    pub admin: Signer<'info>,

    /// AdminConfig to verify the signer is an admin
    #[account(
        has_one = admin @ AppealError::UnauthorizedSigner
    )]
    pub admin_config: Account<'info, AdminConfig>,

    /// Appeal PDA derived from `account` and `bet_key`
    #[account(
        seeds = [b"appeal", bet_account.key().as_ref(), &bet_key.to_le_bytes()],
        bump = appeal.bump
    )]
    pub appeal: Account<'info, Appeal>, // Appeal data stored in the account
}

impl<'info> GetAppeal<'info> {
    pub fn get_appeal(&self) -> Result<&Appeal> {
        let appeal = &self.appeal; // Borrowing appeal data

        // Dereference the appeal Account to get the Appeal struct and clone it
        Ok(appeal) // Cloning the Appeal struct and returning it
    }
}
