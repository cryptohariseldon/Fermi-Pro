use crate::error::OpenBookError;
use crate::pubkey_option::NonZeroKey;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer, Approve},
};

#[derive(Accounts)]
pub struct ConsumeEvents<'info> {
    pub consume_events_admin: Option<Signer<'info>>,
    #[account(
        mut,
        has_one = event_heap,
        constraint = market.load()?.consume_events_admin == consume_events_admin.non_zero_key() @ OpenBookError::InvalidConsumeEventsAdmin
    )]
    pub market: AccountLoader<'info, Market>,
    #[account(mut)]
    pub event_heap: AccountLoader<'info, EventHeap>,
}


#[derive(Accounts)]
pub struct AtomicFinalizeDirect<'info> {
    #[account(
        mut,
        has_one = event_heap,
        //constraint = market.load()?.consume_events_admin == consume_events_admin.non_zero_key() @ OpenBookError::InvalidConsumeEventsAdmin
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(mut)]
    /// CHECK : not usafe.
    pub market_authority:  UncheckedAccount<'info>,

    #[account(mut)]
    pub event_heap: AccountLoader<'info, EventHeap>,
    
    #[account(
        mut,
        token::mint = market_vault_base.mint
    )]
    pub taker_base_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = market_vault_quote.mint
    )]
    pub taker_quote_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = market_vault_base.mint
    )]
    pub maker_base_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = market_vault_quote.mint
    )]
    pub maker_quote_account: Box<Account<'info, TokenAccount>>,


    #[account(mut)]
    pub market_vault_quote: Box<Account<'info, TokenAccount>>, // Market's quote vault

    #[account(mut)]
    pub market_vault_base: Box<Account<'info, TokenAccount>>, // Market's base vault

    ///CHECK: not unsafe.
    #[account(mut)]
    //pub maker: Account<'info, OpenOrdersAccount>, // Maker's OpenOrdersAccount
    pub maker: AccountLoader<'info, OpenOrdersAccount>,
    //pub maker: AccountInfo<'info>, // Maker's EOA

    ///CHECK: not unsafe.
    #[account(mut)]
    //pub maker: Account<'info, OpenOrdersAccount>, // Maker's OpenOrdersAccount
    pub taker: AccountLoader<'info, OpenOrdersAccount>,
    //pub maker: AccountInfo<'info>, // Maker's EOA

    pub token_program: Program<'info, Token>,
    //pub program_id: Program<'info, OpenBook>,
    pub system_program: Program<'info, System>,

}


#[derive(Accounts)]
pub struct AtomicFinalizeGiven<'info> {
    #[account(
        mut,
        has_one = event_heap,
        //constraint = market.load()?.consume_events_admin == consume_events_admin.non_zero_key() @ OpenBookError::InvalidConsumeEventsAdmin
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(mut)]
    /// CHECK : not usafe.
    pub market_authority:  UncheckedAccount<'info>,

    #[account(mut)]
    pub event_heap: AccountLoader<'info, EventHeap>,

    #[account(mut)]
    pub bids: AccountLoader<'info, BookSide>,
    #[account(mut)]
    pub asks: AccountLoader<'info, BookSide>,

    
    #[account(mut)]
    pub maker_ata: Account<'info, TokenAccount>, // Maker's ATA

    #[account(mut)]
    pub taker_ata: Account<'info, TokenAccount>, // Taker's ATA

    #[account(mut)]
    pub market_vault_quote: Account<'info, TokenAccount>, // Market's quote vault

    #[account(mut)]
    pub market_vault_base: Account<'info, TokenAccount>, // Market's base vault

    ///CHECK: not unsafe.
    #[account(mut)]
    //pub maker: Account<'info, OpenOrdersAccount>, // Maker's OpenOrdersAccount
    pub maker: AccountLoader<'info, OpenOrdersAccount>,
    //pub maker: AccountInfo<'info>, // Maker's EOA

    ///CHECK: not unsafe.
    #[account(mut)]
    //pub maker: Account<'info, OpenOrdersAccount>, // Maker's OpenOrdersAccount
    pub taker: AccountLoader<'info, OpenOrdersAccount>,
    //pub maker: AccountInfo<'info>, // Maker's EOA

    pub token_program: Program<'info, Token>,
    //pub program_id: Program<'info, OpenBook>,
    pub system_program: Program<'info, System>,

}


#[derive(Accounts)]
pub struct MarketDirectFinalize<'info>{

    pub signer: Signer<'info>,
    #[account(
        mut,
        has_one = event_heap,
        //constraint = market.load()?.consume_events_admin == consume_events_admin.non_zero_key() @ OpenBookError::InvalidConsumeEventsAdmin
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(mut)]
    /// CHECK : not usafe.
    pub market_authority:  UncheckedAccount<'info>,

    #[account(mut)]
    pub event_heap: AccountLoader<'info, EventHeap>,

    #[account(mut)]
    pub bids: AccountLoader<'info, BookSide>,
    #[account(mut)]
    pub asks: AccountLoader<'info, BookSide>,

    #[account(
        mut,
        token::mint = market_vault_base.mint
    )]
    pub taker_base_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = market_vault_quote.mint
    )]
    pub taker_quote_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = market_vault_base.mint
    )]
    pub maker_base_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = market_vault_quote.mint
    )]
    pub maker_quote_account: Box<Account<'info, TokenAccount>>,


    #[account(mut)]
    pub market_vault_quote: Box<Account<'info, TokenAccount>>, // Market's quote vault

    #[account(mut)]
    pub market_vault_base: Box<Account<'info, TokenAccount>>, // Market's base vault

    ///CHECK: not unsafe.
    #[account(mut)]
    //pub maker: Account<'info, OpenOrdersAccount>, // Maker's OpenOrdersAccount
    pub maker: AccountLoader<'info, OpenOrdersAccount>,
    //pub maker: AccountInfo<'info>, // Maker's EOA

    ///CHECK: not unsafe.
    #[account(mut)]
    //pub maker: Account<'info, OpenOrdersAccount>, // Maker's OpenOrdersAccount
    pub taker: AccountLoader<'info, OpenOrdersAccount>,
    //pub maker: AccountInfo<'info>, // Maker's EOA

    pub token_program: Program<'info, Token>,
    //pub program_id: Program<'info, OpenBook>,
    pub system_program: Program<'info, System>,
}