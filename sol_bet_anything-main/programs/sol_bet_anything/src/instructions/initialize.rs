use anchor_lang::prelude::*;

use crate::state::AdminConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = AdminConfig::INIT_SPACE,
        seeds = [b"admin_config"],
        bump
    )]
    pub admin_config: Account<'info, AdminConfig>,

    #[account(
        seeds = [b"treasury", admin_config.key().as_ref()], 
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, payout_fee: u8, appeal_fee: u64, bumps: &InitializeBumps) -> Result<()> {
        self.admin_config.set_inner(AdminConfig {
            admin: self.admin.key(),
            payout_fee,
            appeal_fee,
            next_bet_key: 1, // Starting counter for bet keys
            treasury_bump: bumps.treasury,
            bump: bumps.admin_config,
        });
        Ok(())
    }
}
