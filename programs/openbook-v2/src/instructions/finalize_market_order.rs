use std::sync::atomic;

use anchor_lang::prelude::*;
use bytemuck::cast_ref;
use itertools::Itertools;

use crate::error::OpenBookError;
use crate::state::*;
use crate::token_utils::token_transfer_signed;

use crate::accounts_ix::*;
use anchor_spl::token::TokenAccount;

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
                msg!("loaded {} account {}", stringify!($name), $key.to_string());
                let ooa: AccountLoader<OpenOrdersAccount> = AccountLoader::try_from(ai)?;
                ooa
                
            }
        };
        let mut $name = loader.load_mut()?;
    };
}

//key difference is - transfer from maker to taker, and from market vault to maker.
#[allow(clippy::too_many_arguments)]

pub fn atomic_finalize_market(
    ctx: Context<AtomicFinalizeDirect>,
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
    let maker_base_account = &ctx.accounts.maker_base_account;
    let maker_quote_account = &ctx.accounts.maker_quote_account;
    let taker_base_account = &ctx.accounts.taker_base_account;
    let taker_quote_account = &ctx.accounts.taker_quote_account;
    let maker_account = ctx.accounts.maker.to_account_info();

    let token_program = &ctx.accounts.token_program;
    
    let program_id = ctx.program_id;
    let remaining_accs = [
        ctx.accounts.maker.to_account_info(),
        ctx.accounts.taker.to_account_info(),
    ];
    let market_authority = &ctx.accounts.market_authority;
    
    let slots_to_consume = [slots.unwrap_or_default()];
    msg!("slots: {:?}", slots_to_consume);
    
    for slot in slots_to_consume {
        let event: &AnyEvent = event_heap.at_slot(slot).unwrap();
        msg!("event is {}", event.event_type);
        

        match EventType::try_from(event.event_type).map_err(|_| error!(OpenBookError::SomeError))? {
            EventType::Fill => {
                let fill: &FillEvent = cast_ref(event);
                panic!("Fill event not supported in atomic_finalize_market - use atomic_finalize.");
            }
            EventType::FillDirect => {
                let fill: &FillEventDirect = cast_ref(event);

                // TODO: FUNCT WO LOADING OPENORDERS
                // Assuming execute_maker_atomic and execute_taker_atomic are defined
                msg!("maker: {}", fill.maker.to_string());
                load_open_orders_account!(maker, maker_account.key(), remaining_accs);
                
                //NO openorders associated with taker
                //load_open_orders_account!(taker, fill.taker, remaining_accs);

                
                msg!("execute maker atomic");
                // borrow issues with this line. attempting mannual transfers
               
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
                       
                        quote_amount_transfer = quote_amount;
                        // fixed debit from locked tokens
                        

                        // Calculate base amount and transfer for a Bid
                        base_amount = (fill.quantity * market.base_lot_size) as u64;

                        let base_amount_remaining = base_amount - (base_amount / 100);
                        // base_lots in OO position being used to settle 1% of trade value.
                        let base_lots_locked = base_amount_remaining as i64 / market.base_lot_size;
                        msg!("base amount locked: {}", base_lots_locked);
                        require!(
                            maker.position.asks_base_lots >= base_lots_locked as i64,
                            OpenBookError::MissingMargin
                        );
                        maker.position.asks_base_lots -= base_lots_locked as i64;

                        //debit from openorders balance as applicable. Variable debit from unlocked tokens.
                        if base_amount_remaining > maker.position.base_free_native {
                            base_amount_transfer =
                                base_amount_remaining - maker.position.base_free_native;
                            maker.position.base_free_native = 0;
                        } else {
                            maker.position.base_free_native -= base_amount_remaining;
                            base_amount_transfer = 0;
                        };
                        // base_amount_transfer = base_amount.checked_sub(maker.position.base_free_native);
                    }
                    Side::Bid => {
                        // Calculate quote amount and transfer for an Ask
                        quote_amount = (fill.quantity * fill.price * market.quote_lot_size) as u64;

                        // Debit 1% margin requirement from bid quote lots
                        let quote_lots_locked = quote_amount as i64 / market.quote_lot_size;
                        msg!("quote lots locked: {}", quote_lots_locked);
                        require!(
                            maker.position.bids_quote_lots >= quote_lots_locked as i64,
                            OpenBookError::MissingMargin
                        );
                        maker.position.bids_quote_lots -= quote_lots_locked as i64;

                        let quote_amount_locked = quote_lots_locked as i64 * market.quote_lot_size;
                        let quote_amount_remaining =
                            quote_amount - (quote_amount_locked as u64 / 100);

                        // Debit fromOpenOrders balance as applicable
                        if quote_amount_remaining > maker.position.quote_free_native {
                            quote_amount_transfer =
                                quote_amount_remaining - maker.position.quote_free_native;
                            maker.position.quote_free_native = 0;
                        } else {
                            maker.position.quote_free_native -= quote_amount_remaining;
                            quote_amount_transfer = 0;
                        }

                        // Calculate base amount and transfer for an Ask
                        base_amount = (fill.quantity * market.base_lot_size) as u64;

                        // Debit 1% margin requirement from ask base lots
                        base_amount_transfer = base_amount;
                    }
                };

                //transfer both sides:
                // Bidder sends quote, ASKER sends base

                // side  = maker side
                // taker sends from vault, maker from wallet.
                
                let (from_account_base, to_account_base) = match side {
                    Side::Ask => (maker_base_account, taker_base_account),
                    Side::Bid => (market_base_vault, maker_base_account),
                };
                //let to_account_base = market_base_vault;

                // if maker is ASK, maker sends base, gets quote. If maker is BID, maker sends quote, gets base
                let (from_account_quote, to_account_quote) = match side {
                    Side::Ask => (market_quote_vault, maker_quote_account),
                    Side::Bid => (maker_quote_account, taker_quote_account),
                };

                

                let seeds = market_seeds!(market, ctx.accounts.market.key());
                msg!(
                    "transferrring {} tokens from user's ata {} to destination {}",
                    base_amount_transfer,
                    from_account_base.to_account_info().key(),
                    to_account_base.to_account_info().key()
                );
                // Perform the transfer if the amount is greater than zero
                if base_amount_transfer > 0 {
                    msg!("{} tokens of base mint {} transferring from user's account {} to destination {}", base_amount_transfer,  from_account_base.mint, from_account_base.key(), market_base_vault.key());

                    //verify delegated amount
                    msg!(
                        "delegated amount: {}, required amount: {}",
                        from_account_base.delegated_amount,
                        base_amount_transfer
                    );
                    // transfer base token
                    token_transfer_signed(
                        base_amount_transfer,
                        &ctx.accounts.token_program,
                        from_account_base.as_ref(),
                        to_account_base.as_ref(),
                        &ctx.accounts.market_authority,
                        seeds,
                    )?;
                    // Bid recieves base, ASKER recieves quote
                    // credit base to counterparty
                } else {
                    msg!("base transfer amount is 0");
                }
                //transfer quote token
                if quote_amount_transfer > 0 {
                    msg!("{} tokens of quote mint {} transferring from user's account {} to destination {}", quote_amount_transfer,  from_account_quote.mint, from_account_quote.key(), to_account_quote.key());
                    //verify delagated amount
                    msg!(
                        "delegated amount: {}, required amount: {}",
                        from_account_quote.delegated_amount,
                        quote_amount_transfer
                    );
                    token_transfer_signed(
                        quote_amount_transfer,
                        &ctx.accounts.token_program,
                        from_account_quote.as_ref(),
                        to_account_quote.as_ref(),
                        &ctx.accounts.market_authority,
                        seeds,
                    )?;
                    // Bid recieves base, ASKER recieves quote
                    // credit quote to counterparty
                } else {
                    msg!("quote transfer amount is 0");
                }

                
                msg!("token_program: {}", token_program.to_account_info().key);
                
                msg!("completed transfer");
                
            }
            EventType::Out => {
                let out: &OutEvent = cast_ref(event);
                msg!("out event");
                // Assuming a custom function for handling Out events atomically
                //execute_out_atomic(&mut market, out, remaining_accs)?;
            } //}
        }

        msg!("deleting event slot");
        // TODO: consume this event
        // clear pointers
        event_heap.delete_slot(slot)?;
        // clear node added to delete_slot.
    }

    Ok(())
}
