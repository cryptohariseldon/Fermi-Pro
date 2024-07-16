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

pub fn new_order_and_finalize(
    ctx: Context<MarketDirectFinalize>,
    limit: usize,
    orderid: u128,
    qty: u64,
    side: Side,
) -> Result<()> {
    //insert check event type is fill
    msg!("Atomic Finalize Events");
    //require!(event::event_type == EventType::Fill as u8, ErrorCode::UnsupportedEventType);
    msg!("eventheap account is {}", ctx.accounts.event_heap.key());
    let mut market = ctx.accounts.market.load_mut()?;
    
    //check if needed?
    let mut event_heap = ctx.accounts.event_heap.load_mut()?;

    //let market_base_vault = &ctx.accounts.market_vault_base;
    //let market_quote_vault = &ctx.accounts.market_vault_quote;
    let maker_base_account = &ctx.accounts.maker_base_account;
    let maker_quote_account = &ctx.accounts.maker_quote_account;
    let taker_base_account = &ctx.accounts.taker_base_account;
    let taker_quote_account = &ctx.accounts.taker_quote_account;
    let token_program = &ctx.accounts.token_program;
    //let bids = &ctx.accounts.bids;
    //let asks = &ctx.accounts.asks;
    //let market = &ctx.accounts.market;
    //let market_pda = market_account_info; //.key
    //let (order_tree, root) = {
    let mut bids = ctx.accounts.bids.load_mut()?;
    let mut asks = ctx.accounts.asks.load_mut()?;


    

    let program_id = ctx.program_id;
    let remaining_accs = [
        ctx.accounts.maker.to_account_info(),
        ctx.accounts.taker.to_account_info(),
    ];
    let market_authority = &ctx.accounts.market_authority;
    let order_id: u128 = orderid;
    //let mut matchedorder;
    let (matched_quantity, matched_price, component) = if side == Side::Bid {
        //let root = ;
        bids.nodes.find_by_key(bids.root(BookSideOrderTree::Fixed), order_id)
            .map(|node| (node.quantity, node.price_data(), BookSideOrderTree::Fixed))
            .ok_or(OpenBookError::OrderIdNotFound)?
    } else {
        let root = asks.root(BookSideOrderTree::Fixed);
        asks.nodes.find_by_key(root, order_id)
            .map(|node| (node.quantity, node.price_data(), BookSideOrderTree::Fixed))
            .ok_or(OpenBookError::OrderIdNotFound)?
    };

    require!(matched_quantity >= qty as i64, OpenBookError::InsufficientFunds);

    // Calculate the new quantity
    let new_quantity = matched_quantity - qty as i64;

    // Update or remove the order
    if new_quantity > 0 {
        if side == Side::Bid {
            let root = *bids.root_mut(BookSideOrderTree::Fixed);
            if let Some(node) = bids.nodes.find_by_key_mut(&root, order_id) {
                node.quantity = new_quantity;
            }
        } else {
            let root = *asks.root_mut(BookSideOrderTree::Fixed);
            if let Some(node) = asks.nodes.find_by_key_mut(&root, order_id) {
                node.quantity = new_quantity;
            }
        }
    } else {
        if side == Side::Bid {
            let mut root = *bids.root_mut(BookSideOrderTree::Fixed);
            bids.nodes.remove_by_key(&mut root, order_id);
        } else {
            let mut root = *asks.root_mut(BookSideOrderTree::Fixed);
            asks.nodes.remove_by_key(&mut root, order_id);
        }
    }


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
        msg!("{} tokens of quote mint {} transferring from user's account {} to market's vault {}", quote_amount_transfer,  from_account_quote.mint, from_account_quote.key(), to_account_quote.key());
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

    Ok(())
}
