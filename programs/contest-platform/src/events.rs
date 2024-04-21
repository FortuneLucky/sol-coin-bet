use anchor_lang::prelude::{Pubkey, *};

#[event]
pub struct CreateContest {
    pub id: u16,
    pub start_time: u64,
    pub end_time: u64,
    pub alice: Pubkey,
    pub bob_coin: Pubkey,
}
