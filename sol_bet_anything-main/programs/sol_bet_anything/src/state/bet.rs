use anchor_lang::prelude::*;

#[account]
pub struct Bet {
    pub account: Pubkey, // Bettor's public key
    pub bet_key: u64,    // Linked game (List) key
    pub option: u8,      // Option chosen (0 for A, 1 for B)
    pub bump: u8,        // Bump for this PDA
}

impl Space for Bet {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1 + 1;
}
