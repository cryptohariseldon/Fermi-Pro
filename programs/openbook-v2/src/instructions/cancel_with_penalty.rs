use std::sync::atomic;

use anchor_lang::prelude::*;
use bytemuck::cast_ref;
use itertools::Itertools;

use crate::error::FermiError;
use crate::error::OpenBookError;

use crate::accounts_ix::*;
use crate::state::*;
use crate::token_utils::token_transfer_signed;
use anchor_spl::token::TokenAccount;
//use super::{BookSideOrderTree, FillEvent, LeafNode, Market, Side, SideAndOrderTree};
use anchor_spl::token::{self, Transfer};

// Max events to consume per ix.
pub const MAX_EVENTS_CONSUME: usize = 8;

macro_rules! load_open_orders_account {
    ($name:ident, $key:expr, $ais:expr) => {
        let loader = match $ais.iter().find(|ai| ai.key == &$key) {
            None => {
                msg!(
                    "Unable to find {} account {}, skipping",
                    stringify!($name),
                    $key.to_string()
                );
                return Err(FermiError::OpenOrdersError.into()); // Account not found
            }

            Some(ai) => {
                let ooa: AccountLoader<OpenOrdersAccount> = AccountLoader::try_from(ai)?;
                ooa
            }
        };
        let mut $name = loader.load_mut()?;
    };
}

/// Load a open_orders account by key from the list of account infos.
///
/// Message and return Ok() if it's missing, to lock in successful processing
/// of previous events.
///
macro_rules! load_open_orders_account_two {
    ($name:ident, $key:expr, $ais:expr) => {
        let loader = match $ais.iter().find(|ai| ai.key == &$key) {
            None => {
                msg!(
                    "Unable to find {} account {}, skipping",
                    stringify!($name),
                    $key.to_string()
                );
                continue;
            }

            Some(ai) => {
                let ooa: AccountLoader<OpenOrdersAccount> = AccountLoader::try_from(ai)?;
                ooa
            }
        };
        let mut $name = loader.load_mut()?;
    };
}


// Fermi pro
pub fn cancel_with_penalty(ctx: Context<CancelWithPenalty>, side: Side, slot: usize) -> Result<()> {
    
    let mut event_heap = ctx.accounts.event_heap.load_mut()?;

    let event: &AnyEvent = event_heap.at_slot(slot).unwrap();

    let fill: &FillEvent = cast_ref(event);

    let mut market = ctx.accounts.market.load_mut()?;

    let remaining_accs = &ctx.remaining_accounts;
    let market_base_vault = &ctx.accounts.market_vault_base;
    let market_quote_vault = &ctx.accounts.market_vault_quote;
    let maker_ata = &ctx.accounts.maker_ata;
    let taker_ata = &ctx.accounts.taker_ata;
    let token_program = &ctx.accounts.token_program;
    
    let program_id = ctx.program_id;
    let remaining_accs = [
        ctx.accounts.maker.to_account_info(),
        ctx.accounts.taker.to_account_info(),
    ];
    let market_authority = &ctx.accounts.market_authority;
    
    let market_base_vault = &ctx.accounts.market_vault_base;
    let market_quote_vault = &ctx.accounts.market_vault_quote;
    let maker_ata = &ctx.accounts.maker_ata;
    let taker_ata = &ctx.accounts.taker_ata;
    let token_program = &ctx.accounts.token_program;
    
    let program_id = ctx.program_id;
    let remaining_accs = [
        ctx.accounts.maker.to_account_info(),
        ctx.accounts.taker.to_account_info(),
    ];
    let market_authority = &ctx.accounts.market_authority;

    let quote_amount = (fill.quantity * fill.price) as u64;
    let base_amount = (fill.quantity * fill.price) as u64;

    // Calculate the penalty amount (1% of deposit_amount)
    //let penalty_amount = deposit_amount / 100;

    // require the mandated delay period has been exceeded
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;
    let event1_timestamp = fill.timestamp;
    require!(
        current_timestamp > event1_timestamp + 60,
        FermiError::FinalizeNotExpired //FinalizeNotExpired
    );
    msg!("loaded accounts");
    load_open_orders_account!(cpty, fill.maker, remaining_accs);
    load_open_orders_account!(owner, fill.taker, remaining_accs);

    // verify counterparty does not have sufficient funds in openorders
    let transfer_amount_owner;
    match side {
        Side::Bid => {
            // Base state
            transfer_amount_owner = quote_amount - owner.position.quote_free_native;
            let transfer_amount_cpty = base_amount - cpty.position.base_free_native;
            require!(
                cpty.position.base_free_native < base_amount,
                FermiError::FinalizeFundsAvailable
            ); //FundsAvailable
            let penalty_amount = transfer_amount_cpty / 100;
            owner.position.quote_free_native += transfer_amount_owner;

            // Deduct the penalty from the cpty's deposit
            cpty.position.asks_base_lots -= i64::try_from(penalty_amount).unwrap();

            // credit penalty to honest counterparty (assuming verification below succeeds)
            owner.position.base_free_native += penalty_amount;
        }
        Side::Ask => {
            // Base State
            transfer_amount_owner = base_amount - owner.position.base_free_native;
            let transfer_amount_cpty = quote_amount - cpty.position.quote_free_native;
            require!(
                cpty.position.quote_free_native < quote_amount,
                FermiError::FinalizeFundsAvailable
            ); //FundAvailable
            let penalty_amount = transfer_amount_cpty / 100;
            owner.position.base_free_native += transfer_amount_owner;

            // Deduct the penalty from the cpty's deposit
            cpty.position.bids_quote_lots -= i64::try_from(penalty_amount).unwrap();

            // credit penalty to honest counterparty (assuming verification below succeeds)
            owner.position.quote_free_native += penalty_amount;
        }
    }

    

    // verfiy honest counterparty by transferring funds if not already present.
    let (from_account, to_account) = match side {
        Side::Ask => (taker_ata, market_base_vault),
        Side::Bid => (maker_ata, market_quote_vault),
    };
    
    let seeds = market_seeds!(market, ctx.accounts.market.key());
    msg!(
        "transferrring {} tokens from user's ata {} to market's vault {}",
        transfer_amount_owner,
        from_account.to_account_info().key(),
        to_account.to_account_info().key()
    );
    // Perform the transfer if the amount is greater than zero
    if transfer_amount_owner > 0 {
        token_transfer_signed(
            transfer_amount_owner,
            &ctx.accounts.token_program,
            &ctx.accounts.taker_ata,
            &ctx.accounts.market_vault_base,
            &ctx.accounts.market_authority,
            seeds,
        )?;
        // already credited to openorders, can be withdrawn by honest counterparty

        //remove event from ob
    }
    event_heap.delete_slot(slot)?;

    Ok(())
}
