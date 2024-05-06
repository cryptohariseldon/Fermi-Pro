use std::sync::atomic;

use anchor_lang::prelude::*;
use bytemuck::cast_ref;
use itertools::Itertools;

use crate::error::OpenBookError;
use crate::error::FermiError;

use crate::state::*;
use crate::token_utils::token_transfer_signed;
use crate::accounts_ix::*;
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

/*
// Old version of cancel_with_penalty
 pub fn cancel_with_penalty(
        ctx: Context<CancelWithPenalty>,
        side: Side,
        event_slot1: u8,
        event_slot2: u8,
        //open_orders_bidder: &mut Account<'info, OpenOrders>,
        //open_orders_asker: &mut Account<'info, OpenOrders>,
        //deposit_amount: u64,
    ) -> Result<()> {
        let open_orders_bidder = &mut ctx.accounts.open_orders_bidder;
        let open_orders_asker = &mut ctx.accounts.open_orders_asker;
        let event_q = &mut ctx.accounts.event_q.load_mut()?;
        let event1: Event = event_q.buf[usize::from(event_slot1)];
        let event2: Event = event_q.buf[usize::from(event_slot2)];
    
        // Calculate the penalty amount (1% of deposit_amount)
        //let penalty_amount = deposit_amount / 100;
        
        // require the mandated delay period has been exceeded
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;
        let event1_timestamp = event1.timestamp;
        let event2_timestamp = event2.timestamp;
        require!(
            current_timestamp > event1_timestamp + 60 && current_timestamp > event2_timestamp + 60,
            FermiError::FinalizeNotExpired
        );

        //Verify that the events are a match.
        require!(event1.order_id_second == event2.order_id || event2.order_id_second == event1.order_id, FermiError::Error);


        // verify that the events have not already been finalized
        require!(event1.finalised == 0 || event2.finalised == 0, FermiError::EventFinalised);

        // Verify openorders specified match the events.
        msg!("event1 owner is {}", event1.owner);
        msg!("openorders bidder is {}", open_orders_bidder.key());
        msg!("event2 owner is {}", event2.owner);
        msg!("openorders asker is {}", open_orders_asker.key());
        require!(open_orders_bidder.key() == event1.owner || open_orders_asker.key() == event1.owner, FermiError::InvalidAuthority);
        //verify counterparty

        require!(open_orders_asker.key() == event2.owner || open_orders_bidder.key() == event2.owner, FermiError::InvalidAuthority);
        
        match side{
            Side::Bid => {
                //verify event1 is not already finalized
                
                //require!(EventFlag::flags_to_side(event1.event_flags) == Side::Bid, FermiError::WrongSide);
                //verify owner of openorders is the bidder

                
                if open_orders_bidder.key() == event1.owner {
                // this ensures that a party cannot be penalised if they've already supplied capital.
                require!(event1.finalised == 0, FermiError::SideAlreadyFinalised);

                let deposit_amount = event1.native_qty_paid;
                let penalty_amount = deposit_amount / 100;

                // Deduct the penalty from the bidder's deposit
                open_orders_bidder.debit_locked_pc(penalty_amount);
        
                // Add the penalty amount to the asker's open order balance
                open_orders_asker.credit_unlocked_pc(penalty_amount);
        
                msg!("Penalty of {} PC Tokens transferred from bidder to asker", penalty_amount);

                
                //If asker has finalized bid, free up their tokens deposited
                    if event2.finalised == 1 {
                        let asker_deposit_amount = event2.native_qty_paid;
                        open_orders_asker.unlock_coin(asker_deposit_amount);
                    } else {
                        // free up locked funds for honest counterparty
                        let asker_marginal_deposit = event2.native_qty_paid / 100;
                        open_orders_asker.unlock_coin(asker_marginal_deposit);
                    }
                }
                else {
                    require!(event2.finalised == 0, FermiError::SideAlreadyFinalised);
                    let deposit_amount = event2.native_qty_paid;
                    let penalty_amount = deposit_amount / 100;
    
                    // Deduct the penalty from the bidder's deposit
                    open_orders_asker.debit_locked_pc(penalty_amount);
            
                    // Add the penalty amount to the asker's open order balance
                    open_orders_bidder.credit_unlocked_pc(penalty_amount);
            
                    msg!("Penalty of {} PC Tokens transferred from bidder to asker", penalty_amount);
    
                    // free up locked funds for honest counterparty
                    let asker_marginal_deposit = event1.native_qty_released;
                    open_orders_bidder.unlock_coin(asker_marginal_deposit);
                    //if asker has finalized bid, free up their tokens deposited
                    if event1.finalised == 1 {
                        let asker_deposit_amount = event1.native_qty_released;
                        open_orders_bidder.unlock_coin(asker_deposit_amount);
                    } else {
                        // free up margin locked funds for honest counterparty
                        let asker_marginal_deposit = event1.native_qty_released / 100;
                        open_orders_bidder.unlock_coin(asker_marginal_deposit);
                    }
                }

            }
            Side::Ask => {
                //verify event2 is not already finalized
                // this ensures that a party cannot be penalised if they've already supplied capital
                if open_orders_asker.key() == event2.owner {
                    require!(event2.finalised == 0, FermiError::SideAlreadyFinalised);

                    let deposit_amount = event2.native_qty_paid;
                    let penalty_amount = deposit_amount / 100;

                    // Deduct the penalty from the asker's deposit
                    open_orders_asker.debit_locked_coin(penalty_amount);
            
                    // Add the penalty amount to the bidder's open order balance
                    open_orders_bidder.credit_unlocked_coin(penalty_amount);
            
                    msg!("Penalty of {} coins transferred from asker to bidder", penalty_amount);

                    // if bidder has finalized bid, free up their tokens deposited
                    if event1.finalised == 1 {
                        let bidder_deposit_amount = event1.native_qty_paid;
                        open_orders_bidder.unlock_pc(bidder_deposit_amount);
                    } else {
                        // free up margin locked funds for honest counterparty
                        let bidder_marginal_deposit = event1.native_qty_paid / 100;
                        open_orders_bidder.unlock_pc(bidder_marginal_deposit);
                    }

            }
            else {
                require!(event1.finalised == 0, FermiError::SideAlreadyFinalised);

                let deposit_amount = event1.native_qty_paid;
                let penalty_amount = deposit_amount / 100;

                // Deduct the penalty from the asker's deposit
                open_orders_bidder.debit_locked_coin(penalty_amount);
        
                // Add the penalty amount to the bidder's open order balance
                open_orders_asker.credit_unlocked_coin(penalty_amount);
        
                msg!("Penalty of {} coins transferred from asker to bidder", penalty_amount);

                //if bidder has finalized bid, free up their tokens deposited
                if event2.finalised == 1 {
                    let bidder_deposit_amount = event2.native_qty_released;
                    open_orders_asker.unlock_pc(bidder_deposit_amount);
                } else {
                    // free up margin locked funds for honest counterparty
                    let bidder_marginal_deposit = event2.native_qty_released / 100;
                    open_orders_asker.unlock_pc(bidder_marginal_deposit);
                }


            }

        } 
    }

        //replace events with finalised = 2
        let fin: u8 = 2;
        let owner = event1.owner;
        let bidder_fill = Event::new(EventView::Finalise {
            side: Side::Ask,
            maker: true,
            native_qty_paid:  event1.native_qty_paid,
            native_qty_received: event1.native_qty_released,
            order_id: event1.order_id,
            owner: event1.owner,
            owner_slot: event1.owner_slot,
            finalised: fin,
            cpty: owner,
        });
        let idx = event_slot1;
        event_q.buf[idx as usize] = bidder_fill;

        let owner = event2.owner;
        let asker_fill = Event::new(EventView::Finalise {
            side: Side::Ask,
            maker: true,
            native_qty_paid:  event2.native_qty_paid,
            native_qty_received: event2.native_qty_released,
            order_id: event2.order_id,
            owner: event2.owner,
            owner_slot: event2.owner_slot,
            finalised: fin,
            cpty: owner,
        });
        let idx = event_slot2;
        event_q.buf[idx as usize] = asker_fill;

        Ok(())
    } 

*/

// Fermi pro
pub fn cancel_with_penalty(
    ctx: Context<CancelWithPenalty>,
    side: Side,
    slot: usize,
) -> Result<()> {
    //let open_orders_bidder = &mut ctx.accounts.open_orders_bidder;
    //let open_orders_asker = &mut ctx.accounts.open_orders_asker;
    //let event_q = &mut ctx.accounts.event_q.load_mut()?;
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
    //let market = &ctx.accounts.market;
    //let market_pda = market_account_info; //.key
    let program_id = ctx.program_id;
    let remaining_accs = [ctx.accounts.maker.to_account_info(), ctx.accounts.taker.to_account_info()];
    let market_authority = &ctx.accounts.market_authority;
    //let event2: Event = event_q.buf[usize::from(event_slot2)];
    let market_base_vault = &ctx.accounts.market_vault_base;
    let market_quote_vault = &ctx.accounts.market_vault_quote;
    let maker_ata = &ctx.accounts.maker_ata;
    let taker_ata = &ctx.accounts.taker_ata;
    let token_program = &ctx.accounts.token_program;
    //let market = &ctx.accounts.market;
    //let market_pda = market_account_info; //.key
    let program_id = ctx.program_id;
    let remaining_accs = [ctx.accounts.maker.to_account_info(), ctx.accounts.taker.to_account_info()];
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
            require!(cpty.position.base_free_native < base_amount, FermiError::FinalizeFundsAvailable); //FundsAvailable
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
            require!(cpty.position.quote_free_native < quote_amount, FermiError::FinalizeFundsAvailable); //FundAvailable
            let penalty_amount = transfer_amount_cpty / 100;
            owner.position.base_free_native += transfer_amount_owner;

            // Deduct the penalty from the cpty's deposit
            cpty.position.bids_quote_lots -= i64::try_from(penalty_amount).unwrap();

            // credit penalty to honest counterparty (assuming verification below succeeds)
            owner.position.quote_free_native += penalty_amount;


        }
    }
    
    //Verfiy that event is not already finalized / already c w p has been called
    // superflous as the event is already consumed.
    //require!(.finalised == 0, FermiError::EventFinalised);

    // verfiy honest counterparty by transferring funds if not already present.
    let (from_account, to_account) = match side {
        Side::Ask => (taker_ata, market_base_vault),
        Side::Bid => (maker_ata, market_quote_vault),
    };
    // Construct the seeds for the market PDA
    // jit transfers
    //let seeds: &[&[&[u8]]] = &[seeds_slice];
    let seeds = market_seeds!(market, ctx.accounts.market.key());
    msg!("transferrring {} tokens from user's ata {} to market's vault {}", transfer_amount_owner, from_account.to_account_info().key(), to_account.to_account_info().key());
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

   




            



