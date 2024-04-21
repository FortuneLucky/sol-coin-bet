use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
use crate::instructions::{CreateContest, DepostAliceTokenContest, DepostBobTokenContest, WinnerCliam, WithdrawToken, InitTokenAccountForNewContest};
use instructions::*;

declare_id!("CqJqQSsEv35yyHbA1Q4VXjyLRzLXoZkcaxWPsnzR6TCq");

#[program]
pub mod contest_platform {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
        instructions::initialize(ctx, fee)
    }

    pub fn update_fee(ctx: Context<UpdateManage>, fee: u64) -> Result<()> {
        instructions::update_fee(ctx, fee)
    }

    pub fn update_owner(ctx: Context<UpdateManage>, new_owner: Pubkey) -> Result<()> {
        instructions::update_owner(ctx, new_owner)
    }


    pub fn init_token_account_for_new_contest(
        ctx: Context<InitTokenAccountForNewContest>,
        contest_index: u16,
    ) -> Result<()> {
        instructions::init_token_account_for_new_contest(ctx, contest_index)
    }

    pub fn create_contest(
        ctx: Context<CreateContest>,
        contest_index: u16,
        contest_time: u64,
        wait_time: u64,
        max_alice_amount: u64,
        max_bob_amount: u64,
        alice_name: String,
        bob_name: String,
        alice_image: String,
        bob_image: String,
        alice_fee_account: Pubkey,
        bob_fee_account: Pubkey,
        base_alice_vault: Pubkey,
        quote_alice_sol_valut: Pubkey,
        base_bob_vault: Pubkey,
        quote_bob_sol_vault: Pubkey,
    ) -> Result<()> {
        instructions::create_contest(
            ctx, 
            contest_index,
            contest_time, 
            wait_time,
            max_alice_amount,
            max_bob_amount,
            alice_name,
            bob_name,
            alice_image,
            bob_image,
            alice_fee_account,
            bob_fee_account,
            base_alice_vault,
            quote_alice_sol_valut,
            base_bob_vault,
            quote_bob_sol_vault
        )
    }

    pub fn deposit_alice_token_contest(
        ctx: Context<DepostAliceTokenContest>,
        contest_index: u16,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_alice_token_contest(ctx, contest_index, amount)
    }

    pub fn deposit_bob_token_contest(
        ctx: Context<DepostBobTokenContest>,
        contest_index: u16,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_bob_token_contest(ctx, contest_index, amount)
    }


    pub fn winner_claim(ctx: Context<WinnerCliam>, contest_index: u16) -> Result<()> {
        instructions::winner_claim(ctx, contest_index)
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, contest_index: u16) -> Result<()> {
        instructions::withdraw_token(ctx, contest_index)
    }

    pub fn set_winner(
        ctx: Context<SetWinner>,
        contest_index: u16,
    ) -> Result<()> {
        instructions::set_winner(
            ctx,
            contest_index,
        )
    }
}
