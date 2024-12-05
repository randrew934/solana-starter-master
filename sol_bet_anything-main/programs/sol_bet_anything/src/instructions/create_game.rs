use crate::error::BetError;
use crate::state::{AdminConfig, List};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateGameResponse {
    pub name: String,             // Name of the list
    pub description: String,      // Description of the list
    pub bet_key: u64,             // Unique key for bets
    pub options: Vec<String>,     // Flexible number of options (2 to 4)
    pub amount: u64,              // Individual bet amount (default per bet)
    pub bet_period: u64,          // Bet period in seconds
    pub creation_timestamp: i128, // Timestamp of list creation
    pub judge: Pubkey,            // Judge's public key
    pub maker: Pubkey,            // Maker's public key
    pub status: u8,               // Status of the list
}

// Create the instruction struct for the `create_game` instruction
#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, // Admin who is creating the game

    #[account(mut)]
    pub admin_config: Account<'info, AdminConfig>, // Admin config containing the fee and bet key counter

    // The List (Game) to be created
    #[account(
        init,
        payer = maker,
        space = List::INIT_SPACE,
        seeds = [b"list", maker.key().as_ref(), &admin_config.next_bet_key.to_le_bytes()],
        bump
    )]
    pub list: Account<'info, List>,

    #[account(
        seeds = [b"list_treasury", list.key().as_ref()], 
        bump
    )]
    pub list_treasury: SystemAccount<'info>,

    // Other associated accounts
    pub system_program: Program<'info, System>, // System program for transaction
}

impl<'info> CreateGame<'info> {
    pub fn create_game(
        &mut self,
        name: String,
        description: String,
        options: Vec<String>,
        amount: u64,
        bet_period: u64,
        judge: Pubkey,
        bumps: &CreateGameBumps,
    ) -> Result<CreateGameResponse> {
        let admin_config = &mut self.admin_config;
        let list = &mut self.list;

        // Generate a unique bet key by incrementing the counter
        let bet_key = admin_config.next_bet_key;
        admin_config.next_bet_key += 1; // Increment for the next game

        // Ensure options are between 2 and 4
        if options.len() < 2 || options.len() > 4 {
            return Err(BetError::InvalidNumberOfOptions.into());
        }

        // Set status based on whether judge is admin
        if judge == admin_config.admin {
            list.status = 1; // Automatically approved
        } else {
            list.status = 0; // Pending approval
        }

        // Initialize options and option_counts
        let option_counts = vec![0; options.len()]; // Initialize counts for each option
        list.option_counts = option_counts;

        // Set the list's fields
        list.name = name;
        list.description = description;
        list.bet_key = bet_key;
        list.options = options;
        list.amount = amount;
        list.pool_amount = 0; // Initial pool amount is 0
        list.pool_no = 0; // No bets placed yet
        list.bet_period = bet_period;
        list.creation_timestamp = Clock::get()?.unix_timestamp as i128;
        list.judge = judge;
        list.maker = self.maker.key();
        list.winner = 0; // No winner yet
        list.payout = 0; //No payout
        list.appealed = 0; // No appeal initially
        list.declaration_timestamp = 0; // No declaration timestamp
        list.ended_timestamp = 0; // No Ended timestamp yet
        list.close_timestamp = 0; // No close timestamp yet
        list.treasury_bump = bumps.list_treasury; //Bump for the List Treasury account
        list.bump = bumps.list; // Bump for the list account

        // Return the bet_key in the response
        Ok(CreateGameResponse {
            name: list.name.clone(),
            description: list.description.clone(),
            bet_key,
            options: list.options.clone(),
            amount: list.amount,
            bet_period: list.bet_period,
            creation_timestamp: list.creation_timestamp,
            judge: list.judge,
            maker: list.maker,
            status: list.status,
        })
    }
}
