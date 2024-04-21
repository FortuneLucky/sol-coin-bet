use crate::{constants::*, errors::*, state::*};

use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
// use solana_program::{
//     program::{invoke, invoke_signed},
//     system_instruction,
// };

use std::mem::size_of;

pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.owner = accts.admin.key();
    accts.global_state.fee = fee;
    accts.global_state.contest_id = 0;
    accts.global_state.finised_contest = 0;

    Ok(())
}

pub fn update_fee(ctx: Context<UpdateManage>, fee: u64) -> Result<()> {
    let accts = ctx.accounts;

    if accts.global_state.owner != accts.owner.key() {
        return Err(ContestError::NotAllowedOwner.into());
    }

    accts.global_state.fee = fee;
    Ok(())
}

pub fn update_owner(ctx: Context<UpdateManage>, new_owner: Pubkey) -> Result<()> {
    let accts = ctx.accounts;

    if accts.global_state.owner != accts.owner.key() {
        return Err(ContestError::NotAllowedOwner.into());
    }

    accts.global_state.owner = new_owner;
    Ok(())
}

pub fn init_token_account_for_new_contest(
    _ctx: Context<InitTokenAccountForNewContest>,
    _contest_index: u16
    ) -> Result<()> {
    Ok(())
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
    quote_alice_sol_vault: Pubkey,
    base_bob_vault: Pubkey,
    quote_bob_sol_vault: Pubkey,
    
) -> Result<()> {
    let accts = ctx.accounts;

    if accts.global_state.owner != accts.owner.key() {
        return Err(ContestError::NotAllowedOwner.into());
    }

    if accts.global_state.contest_id + 1 != contest_index {
        return Err(ContestError::InvalidContestIndex.into());
    }
    let start_time = accts.clock.unix_timestamp as u64;

    *accts.contest = Contest {
        id: contest_index.into(),
        fee: accts.global_state.fee,
        start_time:  start_time,
        contest_time: contest_time,
        wait_time: wait_time,
        max_alice_amount: max_alice_amount,
        max_bob_amount: max_bob_amount,
        alice_coin: accts.token_mint_alice.key(),
        bob_coin: accts.token_mint_bob.key(),
        alice_name: alice_name,
        bob_name: bob_name,
        alice_image: alice_image,
        bob_image: bob_image,
        winner: accts.token_mint_alice.key(),
        alice_amount: 0,
        alice_decimal: accts.token_mint_alice.decimals,
        bob_amount: 0,
        bob_decimal: accts.token_mint_bob.decimals,
        alice_remain_amount: 0,
        bob_remain_amount: 0,
        alice_sol_amount: 0,
        bob_sol_amount: 0,
        total_alice_bet: 0,
        total_bob_bet: 0,
        set: false,
        alice_fee_account:alice_fee_account,
        bob_fee_account: bob_fee_account,
        base_alice_vault: base_alice_vault,
        quote_alice_sol_vault: quote_alice_sol_vault,
        base_bob_vault: base_bob_vault,
        quote_bob_sol_vault: quote_bob_sol_vault,
    };

    accts.global_state.contest_id += 1;

    Ok(())
}

pub fn deposit_alice_token_contest(
    ctx: Context<DepostAliceTokenContest>,
    contest_index: u16,
    amount: u64,
) -> Result<()> {
    let accts = ctx.accounts;

    if accts.contest.id != contest_index {
        return Err(ContestError::InvalidContestIndex.into());
    }

    if accts.clock.unix_timestamp > (accts.contest.start_time + accts.contest.contest_time).try_into().unwrap() {
        return Err(ContestError::OverTimeContest.into());
    }
    if accts.token_mint.key() != accts.contest.alice_coin {
        return Err(ContestError::InvalidToken.into());
    }

    if accts.contest.max_alice_amount < amount {
        return Err(ContestError::MaxAmount.into());
    }

    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.token_owner_account.to_account_info().clone(),
            to: accts.token_vault_account.to_account_info().clone(),
            authority: accts.user.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx, amount)?;

    let mut bet: u64 = 0;
    let _current_timestamp = accts.clock.unix_timestamp;

    // fetch the total balance of alice token from alice token vault account
    if accts.base_alice_vault.key() != accts.contest.base_alice_vault {
        return Err(ContestError::InvalidToken.into());
    }

    if accts.quote_alice_sol_vault.key() != accts.contest.quote_alice_sol_vault {
        return Err(ContestError::InvalidToken.into());
    }

    let base_alice_amount = accts.base_alice_vault.amount as u128;
    let quote_sol_amount = accts.quote_alice_sol_vault.amount as u128;

    let k: u128 = base_alice_amount * quote_sol_amount;
    let total_deposited_alice_token_amount = accts.contest.alice_amount + amount;
    let delta = k / (base_alice_amount - total_deposited_alice_token_amount as u128) - quote_sol_amount;

    let sol_amount = amount * delta as u64 / total_deposited_alice_token_amount;

    bet = sol_amount * (accts.contest.bob_sol_amount  + 1000000000) / (accts.contest.alice_sol_amount  + 1000000000) ;

    accts.contest.total_alice_bet += bet;
    accts.contest.alice_amount += amount;
    accts.contest.alice_sol_amount += sol_amount as u64;
     
    if accts.contest_info.amount == 0 {
        *accts.contest_info = ContestInfo {
            owner: accts.user.key(),
            contest_id: contest_index.into(),
            token: accts.token_mint.key(),
            amount,
            decimal: accts.token_mint.decimals,
            bet: bet.clone(),
            claim: false,
        };
    } else {
        accts.contest_info.amount += amount;
        accts.contest_info.bet += bet.clone();
    } 
    Ok(())
}

pub fn deposit_bob_token_contest(
    ctx: Context<DepostBobTokenContest>,
    contest_index: u16,
    amount: u64,
) -> Result<()> {
    let accts = ctx.accounts;

    if accts.contest.id != contest_index {
        return Err(ContestError::InvalidContestIndex.into());
    }

    if accts.clock.unix_timestamp > (accts.contest.start_time + accts.contest.contest_time).try_into().unwrap() {
        return Err(ContestError::OverTimeContest.into());
    }

    if accts.token_mint.key() != accts.contest.bob_coin {
        return Err(ContestError::InvalidToken.into());
    }

    if accts.contest.max_bob_amount < amount {
        return Err(ContestError::MaxAmount.into());
    }


    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.token_owner_account.to_account_info().clone(),
            to: accts.token_vault_account.to_account_info().clone(),
            authority: accts.user.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx, amount)?;

    let mut bet: u64 = 0;
    let _current_timestamp = accts.clock.unix_timestamp;

    // fetch the total balance of alice token from alice token vault account
    if accts.base_bob_vault.key() != accts.contest.base_bob_vault {
        return Err(ContestError::InvalidToken.into());
    }

    if accts.quote_bob_sol_vault.key() != accts.contest.quote_bob_sol_vault {
        return Err(ContestError::InvalidToken.into());
    }

    let base_bob_amount = accts.base_bob_vault.amount as u128;
    let quote_sol_amount = accts.quote_bob_sol_vault.amount as u128;

    let k: u128 = base_bob_amount * quote_sol_amount;
    let total_deposited_bob_token_amount = accts.contest.bob_amount + amount;
    let delta = k / (base_bob_amount - total_deposited_bob_token_amount as u128) - quote_sol_amount;

    let sol_amount = amount * delta as u64  / total_deposited_bob_token_amount ;

    bet = sol_amount *  (accts.contest.alice_sol_amount  + 1000000000) / (accts.contest.bob_sol_amount  + 1000000000) ;

    accts.contest.total_bob_bet += bet;
    accts.contest.bob_amount += amount;
    accts.contest.bob_sol_amount += sol_amount;
      
    if accts.contest_info.amount == 0 {
        *accts.contest_info = ContestInfo {
            owner: accts.user.key(),
            contest_id: contest_index.into(),
            token: accts.token_mint.key(),
            amount,
            decimal: accts.token_mint.decimals,
            bet: bet.clone(),
            claim: false,
        };
    } else {
        accts.contest_info.amount += amount;
        accts.contest_info.bet += bet.clone();
    } 
    Ok(())
}

pub fn set_winner(
    ctx: Context<SetWinner>,
    contest_index: u16,
) -> Result<()> {
    let accts = ctx.accounts;

    if accts.contest.start_time + accts.contest.contest_time > accts.clock.unix_timestamp as u64 {
        return Err(ContestError::NotFinished.into());
    }

    if accts.contest.set == true {
        return Err(ContestError::AlreadySet.into());
    }

    if accts.global_state.owner != accts.owner.key() {
        return Err(ContestError::NotAllowedOwner.into());
    }

    if accts.contest.id != contest_index {
        return Err(ContestError::InvalidContestIndex.into());
    }

    if accts.contest.alice_sol_amount > accts.contest.bob_sol_amount {
        accts.contest.winner = accts.contest.alice_coin;
    } else if  accts.contest.alice_sol_amount < accts.contest.bob_sol_amount {
        accts.contest.winner = accts.contest.bob_coin;
    } else {
        let case = (accts.clock.unix_timestamp % 2) as u8;

        match case {
            1_u8 => {
                accts.contest.winner = accts.contest.alice_coin;
            },
            0_u8 => {
                accts.contest.winner = accts.contest.bob_coin;
            },
            _ => todo!()
        }
    }

    accts.contest.set = true;
    accts.contest.alice_remain_amount = accts.contest.alice_amount;
    accts.contest.bob_remain_amount = accts.contest.bob_amount;
    accts.global_state.finised_contest += 1;

    Ok(())
}


pub fn winner_claim(ctx: Context<WinnerCliam>, contest_index: u16) -> Result<()> {
    let accts = ctx.accounts;

    if accts.contest.set != true {
        return Err(ContestError::DidntSetWinner.into());
    }

    if accts.contest.winner != accts.contest_info.token {
        return Err(ContestError::NotWinner.into());
    }

    if accts.contest.id != accts.contest_info.contest_id {
        return Err(ContestError::InvalidContestIndex.into());
    }

    let (_, bump) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], ctx.program_id);
    let vault_seeds = &[GLOBAL_STATE_SEED, &[bump]];
    let signer = &[&vault_seeds[..]];

    
    if accts.contest.winner == accts.contest.alice_coin  {
        if accts.contest_info.claim == true {
            return Err(ContestError::AlreadyClaimed.into());
        }

        let cpi_ctx_alice_fee = CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.token_vault_account_alice.to_account_info().clone(),
                to: accts.token_fee_account_alice.to_account_info().clone(),
                authority: accts.global_state.to_account_info().clone(),
            },
        );

        let alice_fee_amount = accts.contest_info.amount * accts.contest.fee / 100;

        transfer(
            cpi_ctx_alice_fee.with_signer(signer),
            alice_fee_amount,
        )?;
        
        let cpi_ctx = CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.token_vault_account_alice.to_account_info().clone(),
                to: accts.token_owner_account_alice.to_account_info().clone(),
                authority: accts.global_state.to_account_info().clone(),
            },
        );

        let alice_amount = accts.contest_info.amount * (100 - accts.contest.fee) / 100;

        transfer(
            cpi_ctx.with_signer(signer),
            alice_amount,
        )?;

        accts.contest.alice_remain_amount -= accts.contest_info.amount;


        let cpi_ctx_bob = CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.token_vault_account_bob.to_account_info().clone(),
                to: accts.token_owner_account_bob.to_account_info().clone(),
                authority: accts.global_state.to_account_info().clone(),
            },
        );

        let bob_amount = accts.contest.bob_amount * (100 - accts.contest.fee) / 100 * accts.contest_info.bet
        / (accts.contest.total_alice_bet);

        transfer(
            cpi_ctx_bob.with_signer(signer),
            bob_amount,
        )?;
        
        accts.contest.bob_remain_amount -= bob_amount;
    }
    
    if accts.contest.winner == accts.contest.bob_coin  {

        if accts.contest_info.claim == true {
            return Err(ContestError::AlreadyClaimed.into());
        }
        let cpi_ctx_bob_fee = CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.token_vault_account_bob.to_account_info().clone(),
                to: accts.token_fee_account_bob.to_account_info().clone(),
                authority: accts.global_state.to_account_info().clone(),
            },
        );

        transfer(
            cpi_ctx_bob_fee.with_signer(signer),
            accts.contest_info.amount * accts.contest.fee / 100,
        )?;

        
        let cpi_ctx = CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.token_vault_account_bob.to_account_info().clone(),
                to: accts.token_owner_account_bob.to_account_info().clone(),
                authority: accts.global_state.to_account_info().clone(),
            },
        );

        let bob_amount = accts.contest_info.amount * (100 - accts.contest.fee) / 100;

        transfer(
            cpi_ctx.with_signer(signer),
            bob_amount,
        )?;
        
        accts.contest.bob_remain_amount -=  accts.contest_info.amount;

        let cpi_ctx_alice = CpiContext::new(
            accts.token_program.to_account_info(),
            Transfer {
                from: accts.token_vault_account_alice.to_account_info().clone(),
                to: accts.token_owner_account_alice.to_account_info().clone(),
                authority: accts.global_state.to_account_info().clone(),
            },
        );

        let alice_amount = accts.contest.alice_amount * (100 - accts.contest.fee) / 100
        * accts.contest_info.bet
        / (accts.contest.total_bob_bet);

        transfer(
            cpi_ctx_alice.with_signer(signer),
            alice_amount,
        )?;
        accts.contest.alice_remain_amount -= alice_amount;
    }
    
    accts.contest_info.claim = true;

    Ok(())
}

pub fn withdraw_token(ctx: Context<WithdrawToken>, contest_index: u16) -> Result<()> {
    let accts = ctx.accounts;

    if accts.contest.start_time + accts.contest.contest_time + accts.contest.wait_time > accts.clock.unix_timestamp.try_into().unwrap() {
        return Err(ContestError::NotWithdrawTime.into());
    }

    if accts.contest.id != contest_index {
        return Err(ContestError::InvalidContestIndex.into());
    }

    if accts.global_state.owner != accts.owner.key() {
        return Err(ContestError::NotAllowedOwner.into());
    }

    let (_, bump) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], ctx.program_id);
    let vault_seeds = &[GLOBAL_STATE_SEED, &[bump]];
    let signer = &[&vault_seeds[..]];

    let cpi_ctx_alice = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.token_vault_account_alice.to_account_info().clone(),
            to: accts.token_fee_account_alice.to_account_info().clone(),
            authority: accts.global_state.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx_alice.with_signer(signer), accts.contest.alice_remain_amount)?;
    accts.contest.alice_remain_amount = 0;

    let cpi_ctx_bob = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.token_vault_account_bob.to_account_info().clone(),
            to: accts.token_fee_account_bob.to_account_info().clone(),
            authority: accts.global_state.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx_bob.with_signer(signer), accts.contest.bob_remain_amount)?;
    accts.contest.bob_remain_amount = 0;

    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        space = 8 + size_of::<GlobalState>(),
    )]
    pub global_state: Account<'info, GlobalState>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateManage<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(contest_index: u16)]
pub struct InitTokenAccountForNewContest<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    pub token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = owner,
        seeds = [TOKEN_VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = global_state,
    )]
    token_vault_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(contest_index: u16)]
pub struct CreateContest<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        init,
        payer = owner,
        seeds = [CONTEST_CREATE_SEED, &contest_index.to_le_bytes()],
        bump,
        space = 900
    )]
    pub contest: Account<'info, Contest>,

    pub token_mint_alice: Account<'info, Mint>,
    pub token_mint_bob: Account<'info, Mint>,

    #[account(mut)]
    pub base_alice_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_alice_sol_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub base_bob_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_bob_sol_vault: Account<'info, TokenAccount>,
 
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(contest_index: u16)]
pub struct DepostAliceTokenContest<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [CONTEST_CREATE_SEED, &contest_index.to_le_bytes()],
        bump,
    )]
    pub contest: Account<'info, Contest>,

    #[account(
        init_if_needed,
        seeds = [CONTEST_INFO_SEED, &contest_index.to_le_bytes(), user.key().as_ref(), token_mint.key().as_ref()],
        bump,    
        payer = user,
        space = 8 + size_of::<ContestInfo>()
    )]
    pub contest_info: Account<'info, ContestInfo>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_owner_account: Account<'info, TokenAccount>,

    #[account(
        mut,        
        token::mint = token_mint,
        token::authority = global_state,
    )]
    token_vault_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
    )]
    pub base_alice_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
    )]
    pub quote_alice_sol_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}


#[derive(Accounts)]
#[instruction(contest_index: u16)]
pub struct DepostBobTokenContest<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [CONTEST_CREATE_SEED, &contest_index.to_le_bytes()],
        bump,
    )]
    pub contest: Account<'info, Contest>,

    #[account(
        init_if_needed,
        seeds = [CONTEST_INFO_SEED, &contest_index.to_le_bytes(), user.key().as_ref(), token_mint.key().as_ref()],
        bump,    
        payer = user,
        space = 8 + size_of::<ContestInfo>()
    )]
    pub contest_info: Account<'info, ContestInfo>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_owner_account: Account<'info, TokenAccount>,

    #[account(
        mut,        
        token::mint = token_mint,
        token::authority = global_state,
    )]
    token_vault_account: Account<'info, TokenAccount>,
      
    #[account(
        mut,
    )]
    pub base_bob_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
    )]
    pub quote_bob_sol_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(contest_index: u16)]
pub struct WinnerCliam<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        mut,
        seeds = [CONTEST_CREATE_SEED, &contest_index.to_le_bytes()],
        bump,
    )]
    pub contest: Account<'info, Contest>,

    #[account(
        mut,
        seeds = [CONTEST_INFO_SEED, &contest_index.to_le_bytes(), user.key().as_ref(), contest.winner.as_ref()],
        bump,    
    )]
    pub contest_info: Account<'info, ContestInfo>,

    pub token_mint_alice: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = token_mint_alice,
        token::authority = global_state,
    )]
    token_vault_account_alice: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_owner_account_alice: Account<'info, TokenAccount>,
    #[account(
        mut, 
        address = contest.alice_fee_account
    )]
    pub token_fee_account_alice: Account<'info, TokenAccount>,

    pub token_mint_bob: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = token_mint_bob,
        token::authority = global_state,
    )]
    token_vault_account_bob: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_owner_account_bob: Account<'info, TokenAccount>,
    #[account(
        mut,
        address = contest.bob_fee_account
    )]
    pub token_fee_account_bob: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(contest_index: u16)]
pub struct WithdrawToken<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [CONTEST_CREATE_SEED, &contest_index.to_le_bytes()],
        bump,
    )]
    pub contest: Account<'info, Contest>,

    pub token_mint_alice: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = token_mint_alice,
        token::authority = global_state,
    )]
    token_vault_account_alice: Account<'info, TokenAccount>,

    #[account(
        mut, 
        address = contest.alice_fee_account
    )]
    token_fee_account_alice: Account<'info, TokenAccount>,

    pub token_mint_bob: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = token_mint_bob,
        token::authority = global_state,
    )]
    token_vault_account_bob: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = contest.bob_fee_account
    )]
    token_fee_account_bob: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
#[instruction(contest_index: u16)]
pub struct SetWinner<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [CONTEST_CREATE_SEED, &contest_index.to_le_bytes()],
        bump,
    )]
    pub contest: Account<'info, Contest>,

    pub token_mint_alice: Account<'info, Mint>,
    pub token_mint_bob: Account<'info, Mint>,
   
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}