use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub owner: Pubkey, // the pubkey of owner
    pub fee: u64, // the fee of platform
    pub contest_id: u16, // the current contest index 
    pub finised_contest: u16,
}

#[account]
#[derive(Default)]
pub struct Contest {
    pub id: u16, // unique index of contest
    pub fee: u64, // the fee of contest
    pub start_time: u64, // start time of contest
    pub contest_time: u64, // end time of contest
    pub wait_time: u64, // waiting time after the contest has been finished
    pub max_alice_amount: u64, // the max amount that user can deposit alice token each contest
    pub max_bob_amount: u64, // the max amount that user can deposit bob token each contest
    pub alice_coin: Pubkey, // pubkey of alice coin
    pub bob_coin: Pubkey, // pubkey of bob coin
    pub alice_name: String, // the token name of alice
    pub bob_name: String, // the token name of bob
    pub alice_image: String, // the token image of alice
    pub bob_image: String, // the token image of bob 
    pub winner: Pubkey, // winner coin
    pub alice_amount: u64, // the token amount of alice coin
    pub alice_decimal: u8, // the token decimal of alice coin
    pub bob_amount: u64, // the token amount of bob coin 
    pub bob_decimal: u8, // the token decimal of bob coin
    pub alice_remain_amount: u64,
    pub bob_remain_amount: u64,
    pub alice_sol_amount: u64, // the sol amount of alice coin with sol price
    pub bob_sol_amount: u64, // the sol amount of bob coin with sol price
    pub total_alice_bet: u64, // the total bet value in contest
    pub total_bob_bet: u64, // the total bet value in contest
    pub set: bool, // the variable to set the contest status 
    pub alice_fee_account: Pubkey,
    pub bob_fee_account: Pubkey,
    // getting the token price real time without pyth
    pub base_alice_vault: Pubkey,
    pub quote_alice_sol_vault: Pubkey,
    pub base_bob_vault: Pubkey,
    pub quote_bob_sol_vault: Pubkey,
}


#[account]
#[derive(Default)]
pub struct ContestInfo {
    pub owner: Pubkey, // the owner of each contest info (it means the user information)
    pub contest_id: u16, // the in of contest that the user takes part in
    pub token: Pubkey, // the pubkey of token that the user deposit
    pub amount: u64, //  this means the sol amount each token 
    pub decimal: u8, // the decimal of deposited token
    pub bet: u64, // the amount of bet in contest
    pub claim: bool, // if win, variable that can claim the fund with reward
}
