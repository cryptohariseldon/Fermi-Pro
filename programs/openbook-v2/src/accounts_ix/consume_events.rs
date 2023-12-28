use crate::error::OpenBookError;
use crate::pubkey_option::NonZeroKey;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

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
pub struct AtomicFinalize<'info> {
    #[account(
        mut,
        has_one = event_heap,
        //constraint = market.load()?.consume_events_admin == consume_events_admin.non_zero_key() @ OpenBookError::InvalidConsumeEventsAdmin
    )]
    pub market: AccountLoader<'info, Market>,
    #[account(mut)]
    pub event_heap: AccountLoader<'info, EventHeap>,
    
    #[account(mut)]
    pub maker_ata: Account<'info, TokenAccount>, // Maker's ATA

    #[account(mut)]
    pub taker_ata: Account<'info, TokenAccount>, // Taker's ATA

    #[account(mut)]
    pub market_vault_quote: Account<'info, TokenAccount>, // Market's quote vault

    #[account(mut)]
    pub market_vault_base: Account<'info, TokenAccount>, // Market's base vault

}