use anchor_lang::prelude::*;

#[account]
pub struct AdminConfig {
    pub admin: Pubkey,     // Admin account's public key
    pub payout_fee: u8,    // Fee percentage of the payout (or other fee config)
    pub appeal_fee: u64,   // Fee charged for Appeal in Lamports
    pub next_bet_key: u64, // Counter for the next unique bet key
    pub treasury_bump: u8, // Bump for the treasury PDA (for storing SOL)
    pub bump: u8,          // Bump for this admin config PDA
}

impl Space for AdminConfig {
    const INIT_SPACE: usize = 8 + 32 + 3 + 8 + 8;
}
