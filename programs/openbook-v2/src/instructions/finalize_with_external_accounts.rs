use std::sync::atomic;

use anchor_lang::prelude::*;
use bytemuck::cast_ref;
use itertools::Itertools;

use crate::error::OpenBookError;
use crate::state::*;
use crate::token_utils::*;

use crate::accounts_ix::*;
use solana_program::program::invoke;
use solana_program::instruction::Instruction;

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

pub fn new_order_and_finalize_external(
    ctx: Context<MarketDirectFinalize>,
    limit: usize,
    orderid: u128,
    qty: u64,
    side: Side,
) -> Result<()> {
    //insert check event type is fill
    msg!("Place And Finalize");
    //require!(event::event_type == EventType::Fill as u8, ErrorCode::UnsupportedEventType);
    msg!("eventheap account is {}", ctx.accounts.event_heap.key());
    let mut market = ctx.accounts.market.load_mut()?;

    
    //check if needed?
    //let mut event_heap = ctx.accounts.event_heap.load_mut()?;

    //let market_base_vault = &ctx.accounts.market_vault_base;
    //let market_quote_vault = &ctx.accounts.market_vault_quote;
    let maker_base_account = &ctx.accounts.maker_base_account;
    let maker_quote_account = &ctx.accounts.maker_quote_account;
    let taker_base_account = &ctx.accounts.taker_base_account;
    let taker_quote_account = &ctx.accounts.taker_quote_account;
    let token_program = &ctx.accounts.token_program;
    msg!("loaded normie accounts");
    //let bids = &ctx.accounts.bids;
    //let asks = &ctx.accounts.asks;
    //let market = &ctx.accounts.market;
    //let market_pda = market_account_info; //.key
    //let (order_tree, root) = {
    let mut bids = ctx.accounts.bids.load_mut()?;
    let mut asks = ctx.accounts.asks.load_mut()?;
    msg!("loaded orderbooks (mut)");
    let remaining_accounts = &ctx.remaining_accounts;

    // CPI - call the program at remainig account index 0
    // call function "fetch funds"
    // pass arg. qty and taker base account
    // pass remaining accounts [1:] as accounts
    // XXX XXX
    if !remaining_accounts.is_empty() {
        let cpi_program = remaining_accounts[0].clone();
        let mut cpi_accounts = vec![
            AccountMeta::new(ctx.accounts.taker_base_account.key(), false),
        ];
        
        // Add the rest of the remaining accounts
        for account in &remaining_accounts[1..] {
            cpi_accounts.push(AccountMeta::new(account.key(), false));
        }

        let instruction = Instruction {
            program_id: cpi_program.key(),
            accounts: cpi_accounts,
            data: AnchorSerialize::try_to_vec(&(
                "fetch_funds".to_string(),
                qty,
            ))?,
        };

        invoke(&instruction, remaining_accounts)?;
    }
    else {
        msg!("No remaining accounts");
        // fetch liquidity conventionally, i.e. from approved wallets
    
    //xxx xxx

    

    let program_id = ctx.program_id;
    /*let remaining_accs = [
        ctx.accounts.maker.to_account_info(),
        ctx.accounts.taker.to_account_info(),
    ]; */
    let market_authority = &ctx.accounts.market_authority;
    let order_id: u128 = orderid;
    //let mut matchedorder;
    let (matched_quantity, matched_price) = if side == Side::Bid {
        //let root = ;
        msg!("side bid finding qty and price");
        msg!("Processing order: id={}, side={:?}", order_id, side);
        asks.nodes.find_by_key(asks.root(BookSideOrderTree::Fixed), order_id)
            .map(|node| (node.quantity, node.price_data()))
            .ok_or(OpenBookError::OrderIdNotFound)?
    } else {
        let root = bids.root(BookSideOrderTree::Fixed);
        bids.nodes.find_by_key(root, order_id)
            .map(|node| (node.quantity, node.price_data()))
            .ok_or(OpenBookError::OrderIdNotFound)?
    };

    require!(matched_quantity >= qty as i64, OpenBookError::InsufficientFunds);
    msg!("Matched quantity: {}", matched_quantity);
    msg!("Matched price: {}", matched_price);

    // Calculate the new quantity
    let new_quantity = matched_quantity - qty as i64;

    // Update or remove the order
   // WIP

   // book.new_order_autofill()

    // side = Taker side
    // if taker is selling, taker sends base
    // if taker is buying, taker sends quote

    let (from_account_base, to_account_base) = match side {
        Side::Ask => (taker_base_account, maker_base_account),
        Side::Bid => (maker_base_account, taker_base_account),
    };
    //let to_account_base = market_base_vault;

    // if maker is ASK, maker sends base, gets quote. If maker is BID, maker sends quote, gets base
    let (from_account_quote, to_account_quote) = match side {
        Side::Ask => (maker_quote_account, taker_quote_account),
        Side::Bid => (taker_quote_account, maker_quote_account),
    };


    //consider signatory - if side is bid, then taker is signatory for quote.
    // if side is ask, then tkaker is signatory for base
    msg!("from base: {}", from_account_base.to_account_info().key());

    // trade quantities
    let quote_amount_transfer: u64 = qty * matched_price;
    let base_amount_transfer: u64 = qty;

    /*
    let (from_account, to_account) = match side {
        Side::Ask => (taker_ata, market_base_vault),
        Side::Bid => (maker_ata, market_quote_vault),
    }; */

    // Construct the seeds for the market PDA
    // jit transfers
    // let seeds: &[&[&[u8]]] = &[seeds_slice];

    let seeds = market_seeds!(market, ctx.accounts.market.key());
    msg!("got seeds");
    msg!(
        "transferrring {} tokens from user's ata {} to market's vault {}",
        base_amount_transfer,
        from_account_base.to_account_info().key(),
        to_account_base.to_account_info().key()
    );
    // Perform the transfer if the amount is greater than zero
    if base_amount_transfer > 0 {
        msg!("{} tokens of base mint {} transferring from user's account {} to market's vault {}", base_amount_transfer,  from_account_base.mint, from_account_base.key(), to_account_base.key());

        //verify delegated amount
        msg!(
            "delegated amount: {}, required amount: {}",
            from_account_base.delegated_amount,
            base_amount_transfer
        );
        if side == Side::Bid {
            // if taker side is Bid, then transfer base token using market authority
        // transfer base token
        token_transfer_signed(
            base_amount_transfer,
            &ctx.accounts.token_program,
            from_account_base.as_ref(),
            to_account_base.as_ref(),
            &ctx.accounts.market_authority,
            seeds,
        )?;
        }
        else {
            // if taker side is Ask, then transfer base token using user's authority
            //regular token transfer
            token_transfer(
                base_amount_transfer,
                &ctx.accounts.token_program,
                from_account_base.as_ref(),
                to_account_base.as_ref(),
                &ctx.accounts.signer,
            )?;
        }
        // Bid recieves base, ASKER recieves quote
        // credit base to counterparty
    } else {
        msg!("base transfer amount is 0");
    }
    //transfer quote token
    if quote_amount_transfer > 0 {
        msg!("{} tokens of quote mint {} transferring from user's account {} to market's vault {}", quote_amount_transfer,  from_account_quote.mint, from_account_quote.key(), to_account_quote.key());
        //verify delagated amount
        msg!(
            "delegated amount: {}, required amount: {}",
            from_account_quote.delegated_amount,
            quote_amount_transfer
        );
        if side == Side::Ask {
            // if taker is ask, then transfer quote token using market authority
            // transfer quote token
        token_transfer_signed(
            quote_amount_transfer,
            &ctx.accounts.token_program,
            from_account_quote.as_ref(),
            to_account_quote.as_ref(),
            &ctx.accounts.market_authority,
            seeds,
        )?;
        }
        else {
            //regular token transfer
            // if taker is bid, then transfer quote token using user's authority
            msg!("transferring quote token using user's authority");
            token_transfer(
                quote_amount_transfer,
                &ctx.accounts.token_program,
                from_account_quote.as_ref(),
                to_account_quote.as_ref(),
                &ctx.accounts.signer,
            )?;
        }
        // Bid recieves base, ASKER recieves quote
        // credit quote to counterparty
    } else {
        msg!("quote transfer amount is 0");
    }
}
    msg!("all done");
    Ok(())
}
