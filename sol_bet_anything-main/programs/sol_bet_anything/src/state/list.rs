use anchor_lang::prelude::*;

#[account]
pub struct List {
    pub name: String,                // Name of the list
    pub description: String,         // Description of the list
    pub bet_key: u64,                // Unique key for bets
    pub options: Vec<String>,        // Flexible number of options (2 to 4)
    pub option_counts: Vec<u32>,     // Counter for each option (number of bets per option)
    pub amount: u64,                 // Individual bet amount (default per bet)
    pub pool_amount: u64,            // Total SOL staked across all bets
    pub pool_no: u16,                // Number of bets placed
    pub bet_period: u64,             // Bet period in seconds
    pub creation_timestamp: i128,    // Timestamp of list creation
    pub judge: Pubkey,               // Judge's public key
    pub maker: Pubkey,               // Maker's public key
    pub status: u8,                  // Status of the list
    pub winner: u8,                  // Winner (option_a or option_b)
    pub payout: u64,                 // Payout
    pub appealed: u8,                // Check if decision has been appealed (0 for No and 1 for Yes)
    pub declaration_timestamp: i128, // Timestamp of winner declaration
    pub ended_timestamp: i128,       // Timestamp of game ended
    pub close_timestamp: i128,       // Timestamp of game closed from appeal or further judgement
    pub treasury_bump: u8,           // Bump for the treasury PDA (for storing SOL)
    pub bump: u8,                    // Bump for this list PDA
}

impl Space for List {
    const INIT_SPACE: usize = 8
        + (4 + 32)
        + (4 + 128)
        + (4 + (4 + 32) * 4)
        + (4 + (4 * 4))
        + 8
        + 8
        + 8
        + 2
        + 8
        + 8
        + 16
        + 32
        + 32
        + 1
        + 1
        + 1
        + 1
        + 1
        + 16
        + 16
        + 16;
}
