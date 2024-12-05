pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("BtYYc5eyu3Eg1WPsJTE3mh1yXFeknwH4xyqhKL8qRUzW");

#[program]
pub mod sol_bet_anything {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, payout_fee: u8, appeal_fee: u64) -> Result<()> {
        ctx.accounts.init(payout_fee, appeal_fee, &ctx.bumps)
    }

    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        ctx.accounts.change_admin(new_admin)
    }

    pub fn change_admin_fee(
        ctx: Context<ChangeAdminFee>,
        new_payout_fee: u8,
        new_appeal_fee: u64,
    ) -> Result<()> {
        ctx.accounts
            .change_admin_fee(new_payout_fee, new_appeal_fee)
    }

    pub fn get_game(ctx: Context<GetGame>, bet_key: u64) -> Result<List> {
        ctx.accounts.get_game(bet_key).cloned()
    }

    pub fn create_game(
        ctx: Context<CreateGame>,
        name: String,
        description: String,
        options: Vec<String>,
        amount: u64,
        bet_period: u64,
        judge: Pubkey,
    ) -> Result<CreateGameResponse> {
        ctx.accounts.create_game(
            name,
            description,
            options,
            amount,
            bet_period,
            judge,
            &ctx.bumps,
        )
    }

    pub fn place_bet(ctx: Context<PlaceBet>, options: u8, amount: u64) -> Result<()> {
        ctx.accounts.place_bet(options, amount, &ctx.bumps)?;
        ctx.accounts.send_sol()
    }

    pub fn validate_game(ctx: Context<ValidateGame>) -> Result<()> {
        ctx.accounts.validate_game()
    }

    pub fn end_game(ctx: Context<EndGame>) -> Result<()> {
        ctx.accounts.end_game()
    }

    pub fn close_game(ctx: Context<CloseGame>) -> Result<()> {
        ctx.accounts.close_game()
    }

    pub fn declare_winner(ctx: Context<DeclareWinner>, winner: u8) -> Result<()> {
        ctx.accounts.declare_winner(winner)
    }

    pub fn pay_winner(ctx: Context<PayWinner>) -> Result<()> {
        ctx.accounts.pay_winner()
    }

    pub fn make_appeal(
        ctx: Context<MakeAppeal>,
        bet_key: u64,
        description: String,
        appeal_url: String,
    ) -> Result<()> {
        ctx.accounts
            .make_appeal(bet_key, description, appeal_url, &ctx.bumps)?;
        ctx.accounts.send_sol()
    }

    pub fn get_appeal(
        ctx: Context<GetAppeal>,
        bet_account: Pubkey,
        bet_key: u64,
    ) -> Result<Appeal> {
        ctx.accounts.get_appeal().cloned()
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
}
