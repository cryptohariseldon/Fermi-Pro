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
    slots: Option<Vec<usize>>,
) -> Result<()> {
    //insert check event type is fill
    msg!("Atomic Finalize Events");
    //require!(event::event_type == EventType::Fill as u8, ErrorCode::UnsupportedEventType);
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
    let remaining_accs = [ctx.accounts.maker.to_account_info()];
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
    
    let slots_to_consume = slots
        .unwrap_or_default()
        .into_iter()
        .filter(|slot| !event_heap.nodes[*slot].is_free())
        .chain(event_heap.iter().map(|(_event, slot)| slot))
        .unique()
        .take(limit)
        .collect_vec(); 

    //let slot_to_consume = [slots.unwrap_or_default()];
    msg!("slots: {:?}", slots_to_consume);
    //if slots.is_some(){
    for slot in slots_to_consume {
        
        let event = event_heap.at_slot(slot).unwrap();
        msg!("event is {}", event.event_type);
        //let event = event_heap.at_slot(event_slot).unwrap();
        // check next line
       // msg!("event info qty: {}", event.quantity);

    match EventType::try_from(event.event_type).map_err(|_| error!(OpenBookError::SomeError))? {
        EventType::Fill => {
            let fill: &FillEvent = cast_ref(event);
            // Assuming execute_maker_atomic and execute_taker_atomic are defined
            load_open_orders_account!(maker, fill.maker, remaining_accs);
            //maker.execute_maker_atomic(&mut market, &market_pda, fill, maker_ata.to_account_info(), taker_ata.to_account_info(), &token_program, market_base_vault.to_account_info(), market_quote_vault.to_account_info(), *program_id)?;
            msg!("execute maker atomic");
            // borrow issues with this line. attempting mannual transfers
            //maker.execute_maker_atomic(&ctx, &mut market, fill);
            let program_id = ctx.program_id;
            let side = fill.taker_side().invert_side(); //i.e. maker side

            msg!("JIT");
            let transfer_amount = match side {
                Side::Bid => {
                    // For a bid, calculate the amount in quote currency
                    //let quote_amount = (fill.quantity * fill.price * market.quote_lot_size) as u64;
                    let quote_amount = (fill.quantity * fill.price) as u64;
                    quote_amount // Assuming no free quote amount to subtract
                },
                Side::Ask => {
                    // For an ask, calculate the amount in base currency
                    //let base_amount = (fill.quantity * market.base_lot_size) as u64;
                    let base_amount = (fill.quantity) as u64;
                    base_amount // Assuming no free base amount to subtract
                },
            };
            msg!("transfer amt: {}", transfer_amount);
            msg!("fill.quantity: {}", fill.quantity);
            // Determine the from and to accounts for the transfer
            // REVIEW!
            let (from_account, to_account) = match side {
                Side::Ask => (taker_ata, market_base_vault),
                Side::Bid => (maker_ata, market_quote_vault),
            };
            // Construct the seeds for the market PDA
            // jit transfers
            //let seeds: &[&[&[u8]]] = &[seeds_slice];
            let seeds = market_seeds!(market, ctx.accounts.market.key());
            msg!("transferrring {} tokens from user's ata {} to market's vault {}", transfer_amount, from_account.to_account_info().key(), to_account.to_account_info().key());
            // Perform the transfer if the amount is greater than zero
            if transfer_amount > 0 {

            token_transfer_signed(
                    transfer_amount,
                    &ctx.accounts.token_program,
                    &ctx.accounts.taker_ata,
                    &ctx.accounts.market_vault_quote,
                    &ctx.accounts.market_authority,
                    seeds,
            )?;
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
    
             
                msg!("From: {}", from_account.to_account_info().key);
                msg!("To: {}", to_account.to_account_info().key);
                //msg!("market_pda: {}", market_pda.key);
                msg!("token_program: {}", token_program.to_account_info().key);
                //let cpi_context = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, seeds);
                msg!("invoking transfer");
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
            } 
            else {
                msg!("transfer amount is 0");
                //Ok(())
            }
            msg!("executed maker atomic");

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
    //event_heap.delete_slot(slot)?;
}

    Ok(())
}
/* 
pub fn consume_events(
    ctx: Context<ConsumeEvents>,
    limit: usize,
    slots: Option<Vec<usize>>,
) -> Result<()> {
    let limit = std::cmp::min(limit, MAX_EVENTS_CONSUME);

    let mut market = ctx.accounts.market.load_mut()?;
    let mut event_heap = ctx.accounts.event_heap.load_mut()?;
    let remaining_accs = &ctx.remaining_accounts;

    let slots_to_consume = slots
        .unwrap_or_default()
        .into_iter()
        .filter(|slot| !event_heap.nodes[*slot].is_free())
        .chain(event_heap.iter().map(|(_event, slot)| slot))
        .unique()
        .take(limit)
        .collect_vec();

    for slot in slots_to_consume {
        let event = event_heap.at_slot(slot).unwrap();

        match EventType::try_from(event.event_type).map_err(|_| error!(OpenBookError::SomeError))? {
            EventType::Fill => {
                let fill: &FillEvent = cast_ref(event);
                load_open_orders_account!(maker, fill.maker, remaining_accs);
                maker.execute_maker(&mut market, fill);
            }
            EventType::Out => {
                let out: &OutEvent = cast_ref(event);
                load_open_orders_account!(owner, out.owner, remaining_accs);
                owner.cancel_order(out.owner_slot as usize, out.quantity, *market);
            }
        }

        // consume this event
        event_heap.delete_slot(slot)?;
    }

    Ok(())
} */
