use std::sync::atomic;

use anchor_lang::prelude::*;
use bytemuck::cast_ref;
use itertools::Itertools;

use crate::error::OpenBookError;
use crate::state::*;
use crate::token_utils::token_transfer_signed;

use crate::accounts_ix::*;
use anchor_spl::token::TokenAccount;
//use super::{BookSideOrderTree, FillEvent, LeafNode, Market, Side, SideAndOrderTree};
use anchor_spl::token::{self, Transfer};

// Max events to consume per ix.
pub const MAX_EVENTS_CONSUME: usize = 8;

/// Load a open_orders account by key from the list of account infos.
///
/// Message and return Ok() if it's missing, to lock in successful processing
/// of previous events.
macro_rules! load_open_orders_account {
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


pub fn atomic_finalize_events(
    ctx: Context<AtomicFinalize>,
    limit: usize,
    //slots: Option<Vec<usize>>,
    slots: Option<usize>,
) -> Result<()> {
    //insert check event type is fill
    msg!("Atomic Finalize Events");
    //require!(event::event_type == EventType::Fill as u8, ErrorCode::UnsupportedEventType);
    msg!("eventheap account is {}", ctx.accounts.event_heap.key());
    let mut market = ctx.accounts.market.load_mut()?;
    let mut event_heap = ctx.accounts.event_heap.load_mut()?;
    
    
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
    // maker = openorders
    //let maker = ctx.accounts.maker.load_mut()?;
    // maker = EOA
    //let maker = &ctx.accounts.maker;
    //let remaining_accs = &ctx.remaining_accounts;
    //let market_pda = market.key();

    // Ensure the event slot is valid
    /* 
    if event_heap.nodes[event_slot].is_free() {
        return Err(OpenBookError::InvalidEventSlot.into());
    } */
    /* 
    let slots_to_consume = slots
        .unwrap_or_default()
        .into_iter()
        .filter(|slot| !event_heap.nodes[*slot].is_free())
        .chain(event_heap.iter().map(|(_event, slot)| slot))
        .unique()
        .take(limit)
        .collect_vec(); */

    let slots_to_consume = [slots.unwrap_or_default()];
    msg!("slots: {:?}", slots_to_consume);
    //if slots.is_some(){
    for slot in slots_to_consume {
        
        let event: &AnyEvent = event_heap.at_slot(slot).unwrap();
        msg!("event is {}", event.event_type);
        //let event = event_heap.at_slot(event_slot).unwrap();
        // check next line
       // msg!("event info qty: {}", event.quantity);

    match EventType::try_from(event.event_type).map_err(|_| error!(OpenBookError::SomeError))? {
        EventType::Fill => {
            let fill: &FillEvent = cast_ref(event);
            // Assuming execute_maker_atomic and execute_taker_atomic are defined
            load_open_orders_account!(maker, fill.maker, remaining_accs);
            load_open_orders_account!(taker, fill.taker, remaining_accs);

            //maker.execute_maker_atomic(&mut market, &market_pda, fill, maker_ata.to_account_info(), taker_ata.to_account_info(), &token_program, market_base_vault.to_account_info(), market_quote_vault.to_account_info(), *program_id)?;
            msg!("execute maker atomic");
            // borrow issues with this line. attempting mannual transfers
            //maker.execute_maker_atomic(&ctx, &mut market, fill);
            let program_id = ctx.program_id;
            let side = fill.taker_side().invert_side(); //i.e. maker side

            msg!("JIT");
            // BOTH parties?
          // Declare the variables before the match statement
        let mut quote_amount: u64;
        let mut quote_amount_transfer: u64; // Using Option<u64> for checked_sub result
        let mut base_amount: u64;
        let mut base_amount_transfer: u64; // Using Option<u64> for checked_sub result

        // The match statement to assign values based on the side
        match side {
            Side::Ask => {
                // Calculate quote amount and transfer for a Bid
                quote_amount = (fill.quantity * fill.price * market.quote_lot_size) as u64;
                //quote_amount_transfer = quote_amount.checked_sub(taker.position.quote_free_native);
                
                // fixed debit from locked tokens
                let quote_amount_remaining = quote_amount - (quote_amount / 100);
                //margin related calculations
                //let quote_amount_locked: u64 = quote_amount / 100;
                // quote lots in OO position represents lots for which margin has been posted. Thus margin calculation not needed.
                let quote_lots_locked = quote_amount as i64 / market.quote_lot_size;
                msg!("quote amount locked: {}", quote_lots_locked);
                require!(taker.position.bids_quote_lots >= quote_lots_locked as i64, OpenBookError::MissingMargin);
                taker.position.bids_quote_lots -= quote_lots_locked as i64;

                //let quote_amount_remaining: u64 = quote_amount - quote_lots_locked as u64;

                //debit from openorders balance as applicable. Variable debit from unlocked tokens.
                if quote_amount_remaining > taker.position.quote_free_native {
                    quote_amount_transfer = quote_amount_remaining - taker.position.quote_free_native;
                    taker.position.quote_free_native = 0;
                }
                else {
                    taker.position.quote_free_native -= quote_amount_remaining;
                    quote_amount_transfer = 0;
                }

                // Calculate base amount and transfer for a Bid
                base_amount = (fill.quantity * market.base_lot_size) as u64;

                let base_amount_remaining = base_amount - (base_amount / 100);
                // base_lots in OO position being used to settle 1% of trade value.
                let base_lots_locked = base_amount as i64 / market.base_lot_size;
                msg!("base amount locked: {}", base_lots_locked);
                require!(maker.position.asks_base_lots >= base_lots_locked as i64, OpenBookError::MissingMargin);
                maker.position.asks_base_lots -= base_lots_locked as i64;

                //debit from openorders balance as applicable. Variable debit from unlocked tokens.
                if base_amount_remaining > maker.position.base_free_native {
                    base_amount_transfer = base_amount_remaining - maker.position.base_free_native;
                    maker.position.base_free_native = 0;
                }
                else {
                    maker.position.base_free_native -= base_amount_remaining;
                    base_amount_transfer = 0;
                };
               // base_amount_transfer = base_amount.checked_sub(maker.position.base_free_native);
            },
            Side::Bid => {
                // Calculate quote amount and transfer for an Ask
                quote_amount = (fill.quantity * fill.price * market.quote_lot_size) as u64;
            
                // Debit 1% margin requirement from bid quote lots
                let quote_lots_locked = quote_amount as i64 / market.quote_lot_size;
                msg!("quote lots locked: {}", quote_lots_locked);
                require!(maker.position.bids_quote_lots >= quote_lots_locked as i64, OpenBookError::MissingMargin);
                maker.position.bids_quote_lots -= quote_lots_locked as i64;
            
                let quote_amount_locked = quote_lots_locked as i64 * market.quote_lot_size;
                let quote_amount_remaining = quote_amount - (quote_amount_locked as u64 / 100);

            
                // Debit fromOpenOrders balance as applicable
                if quote_amount_remaining > maker.position.quote_free_native {
                    quote_amount_transfer = quote_amount_remaining - maker.position.quote_free_native;
                    maker.position.quote_free_native = 0;
                } else {
                    maker.position.quote_free_native -= quote_amount_remaining;
                    quote_amount_transfer = 0;
                }
            
                // Calculate base amount and transfer for an Ask
                base_amount = (fill.quantity * market.base_lot_size) as u64;
            
                // Debit 1% margin requirement from ask base lots
                let base_lots_locked = base_amount / market.base_lot_size as u64;
                msg!("base lots locked: {}", base_lots_locked);
                require!(taker.position.asks_base_lots >= base_lots_locked as i64, OpenBookError::MissingMargin);
                taker.position.asks_base_lots -= base_lots_locked as i64;
            
                let base_amount_locked = base_lots_locked as i64 * market.base_lot_size  ;
                let base_amount_remaining = base_amount  - (base_amount_locked as u64 /100) ;
            
                // Debit from OpenOrders balance as applicable
                if base_amount_remaining > taker.position.base_free_native {
                    base_amount_transfer = base_amount_remaining - taker.position.base_free_native;
                    taker.position.base_free_native = 0;
                } else {
                    taker.position.base_free_native -= base_amount_remaining;
                    base_amount_transfer = 0;
                }
            },
            /* 
            Side::Bid => {
                // Calculate quote amount and transfer for an Ask
                quote_amount = (fill.quantity * fill.price * market.quote_lot_size) as u64;
                //quote_amount_transfer = quote_amount.checked_sub(maker.position.quote_free_native);
                // debit from openorders balance as applicable.

                //margin related calculations
                // debit 1% margin requirement from ask quote lots
                let quote_amount_locked = quote_amount / 100;
                msg!("quote amount locked: {}", quote_amount_locked);
                require!(maker.position.bids_quote_lots >= quote_amount_locked as i64, OpenBookError::MissingMargin);
                maker.position.bids_quote_lots -= quote_amount_locked as i64;
                let quote_amount_remaining: u64 = quote_amount - quote_amount_locked;

                if quote_amount_remaining > maker.position.quote_free_native {
                    quote_amount_transfer = quote_amount - maker.position.quote_free_native;
                    maker.position.quote_free_native = 0;
                }
                else {
                    maker.position.quote_free_native -= quote_amount_remaining;
                    quote_amount_transfer = 0;
                }

                // Calculate base amount and transfer for an Ask
                base_amount = (fill.quantity * market.base_lot_size) as u64;
                base_amount_remaining = base_amount - (base_amount / 100);

                // debit 1% margin requirement from ask base lots
                let base_lots
                //base_amount_transfer = base_amount.checked_sub(taker.position.base_free_native);
                if base_amount > taker.position.base_free_native {
                    base_amount_transfer = base_amount - taker.position.base_free_native;
                    taker.position.base_free_native = 0;
                }
                else {
                    taker.position.base_free_native -= base_amount;
                    base_amount_transfer = 0;
                }
            }, */
        };

            //transfer both sides:
            // Bidder sends quote, ASKER sends base

            //msg!("transfer amt: {}", transfer_amount.unwrap());
            //msg!("fill.quantity: {}", fill.quantity);
            // Determine the from and to accounts for the transfer
            // REVIEW!
            //msg!("side: {}", side);
            let from_account_base = match side {
                Side::Ask => maker_ata,
                Side::Bid => taker_ata,
            };
            let to_account_base = market_base_vault;

            let from_account_quote = match side {
                Side::Ask => taker_ata,
                Side::Bid => maker_ata,
            };

            let to_account_quote = market_quote_vault;
            /* 
            let (from_account, to_account) = match side {
                Side::Ask => (taker_ata, market_base_vault),
                Side::Bid => (maker_ata, market_quote_vault),
            }; */
            // Construct the seeds for the market PDA
            // jit transfers
            //let seeds: &[&[&[u8]]] = &[seeds_slice];



            let seeds = market_seeds!(market, ctx.accounts.market.key());
            msg!("transferrring {} tokens from user's ata {} to market's vault {}", base_amount_transfer, from_account_base.to_account_info().key(), market_base_vault.to_account_info().key());
            // Perform the transfer if the amount is greater than zero
            if base_amount_transfer > 0 {
                msg!("{} tokens of base mint {} transferring from user's account {} to market's vault {}", base_amount_transfer,  from_account_base.mint, from_account_base.key(), market_base_vault.key());

                //verify delegated amount
                msg!("delegated amount: {}, required amount: {}", from_account_base.delegated_amount, base_amount_transfer);
                // transfer base token
            token_transfer_signed(
                base_amount_transfer,
                    &ctx.accounts.token_program,
                    from_account_base,
                    to_account_base,
                    &ctx.accounts.market_authority,
                    seeds,
            )?;
            // Bid recieves base, ASKER recieves quote
            // credit base to counterparty
        }
            else {
                msg!("base transfer amount is 0");
            }
            //transfer quote token
            if quote_amount_transfer > 0 {

             msg!("{} tokens of quote mint {} transferring from user's account {} to market's vault {}", quote_amount_transfer,  from_account_quote.mint, from_account_quote.key(), market_quote_vault.key());
             //verify delagated amount
                msg!("delegated amount: {}, required amount: {}", from_account_quote.delegated_amount, quote_amount_transfer);
            token_transfer_signed(
                quote_amount_transfer,
                    &ctx.accounts.token_program,
                    from_account_quote,
                    to_account_quote,
                    &ctx.accounts.market_authority,
                    seeds,
            )?;
            // Bid recieves base, ASKER recieves quote
            // credit quote to counterparty
        }
            else {
                msg!("quote transfer amount is 0");
            }

            // CREDIT the maker and taker with the filled amount
            if side == Side::Bid {
                taker.position.quote_free_native += quote_amount;
                maker.position.base_free_native += base_amount;
            } else {
                maker.position.quote_free_native += quote_amount;
                taker.position.base_free_native += base_amount;

            }


            // Perform the transfer if the amount is greater than zero
            //if transfer_amount > 0 {

                /*transfer_from_user(
                    transfer_amount,
                    &token_program.to_account_info(),
                    &from_account.to_account_info(),
                    &to_account.to_account_info(),
                    &market_pda.into(), // Convert Pubkey to AccountInfo
                    seeds,
                ) */
            
            /* 
            // Perform the transfer if the amount is greater than zero
            if transfer_amount > 0 { */
    
             
            //    msg!("From: {}", from_account.to_account_info().key);
              //  msg!("To: {}", to_account.to_account_info().key);
                //msg!("market_pda: {}", market_pda.key);
                msg!("token_program: {}", token_program.to_account_info().key);
                //let cpi_context = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, seeds);
                msg!("completed transfer");
                //anchor_spl::token::transfer(cpi_context, transfer_amount)?;
                /*match anchor_spl::token::transfer(cpi_context, transfer_amount) {
                    Ok(_) => {
                        msg!("Transfer complete of {}", transfer_amount);
                        msg!("From: {}", from_account.to_account_info().key);
                        msg!("To: {}", to_account.to_account_info().key);
                        //Ok(())
                    },
                    Err(e) => {
                        msg!("Error in transfer: {:?}", e);
                        //Err(e)
                    },
                } */
                //msg!("transfer complete of {}", transfer_amount);
                //msg!("from: {}", from_account.to_account_info().key);
                //msg!("to: {}", to_account.to_account_info().key);
                //Ok(())
            

            //load_open_orders_account!(taker, fill.taker, remaining_accs);
            //execute_taker_atomic(&mut market, fill, remaining_accs)?;
        }
        EventType::Out => {
            let out: &OutEvent = cast_ref(event);
            msg!("out event");
            // Assuming a custom function for handling Out events atomically
            //execute_out_atomic(&mut market, out, remaining_accs)?;
        }
    //}
    }

    msg!("deleting event slot");
    // TODO: consume this event
    // clear pointers
    event_heap.delete_slot(slot)?;
    // clear node added to delete_slot.

}
    
    

    Ok(())
}