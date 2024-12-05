use anchor_lang::prelude::*;

#[account]
pub struct Appeal {
    pub account: Pubkey,     // Account filing the appeal
    pub bet_key: u64,        // Bet's associated key
    pub description: String, // Description of the appeal
    pub appeal_url: String,  // Supporting documentation or URL
    pub bump: u8,            // Bump for this PDA
}

impl Space for Appeal {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1 + (4 + 256) + (4 + 128);
}
