use anchor_lang::prelude::*;
use derivative::Derivative;
use static_assertions::const_assert_eq;
use std::mem::size_of;
//use solana_program::pubkey::Pubkey;
//use spl_associated_token_account::get_associated_token_address;
//use spl_token::associated_token::get_associated_token_address;
use solana_program::{
    program_pack::Pack,
    pubkey::Pubkey,
    
};
//use crate::accounts::AtomicFinalize;
use crate::accounts_ix::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, Transfer, Approve} };
use std::str::FromStr;
use anchor_spl::token::TokenAccount;

use crate::logs::FillLog;
use crate::pubkey_option::NonZeroPubkeyOption;
use crate::{error::*, logs::OpenOrdersPositionLog};
use crate::token_utils::transfer_from_user;

use super::{BookSideOrderTree, FillEvent, LeafNode, Market, Side, SideAndOrderTree};

pub const MAX_OPEN_ORDERS: usize = 24;

const SPL_TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    6, 130, 45, 152, 102, 25, 214, 130, 
    42, 12, 190, 77, 91, 177, 138, 5, 
    191, 68, 241, 12, 25, 68, 101, 95, 
    112, 78, 83, 132, 116, 4, 9, 5,
]);

//const token_program_id: anchor_lang::prelude::Pubkey = SPL_TOKEN_PROGRAM_ID;

//const program_id: Pubkey = Pubkey::from_str("E6cNbXn2BNoMjXUg7biSTYhmTuyJWQtAnRX1fVPa7y5v").unwrap();


const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    140, 198, 27, 245, 201, 60, 141, 52, 
    178, 221, 123, 156, 173, 17, 151, 212, 
    178, 42, 176, 217, 197, 27, 216, 25, 
    111, 124, 65, 79, 188, 64, 25, 41,
]);

pub fn get_associated_token_address(wallet_address: &Pubkey, token_mint: &Pubkey) -> Pubkey {
    let seeds = &[
        wallet_address.as_ref(),
        &SPL_TOKEN_PROGRAM_ID.to_bytes(),
        token_mint.as_ref(),
    ];

    let (associated_token_address, _) = Pubkey::find_program_address(seeds, &SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID);
    associated_token_address
}

#[account(zero_copy)]
#[derive(Debug)]
pub struct OpenOrdersAccount {
    pub owner: Pubkey,
    pub market: Pubkey,

    pub name: [u8; 32],

    // Alternative authority/signer of transactions for a openbook account
    pub delegate: NonZeroPubkeyOption,

    pub account_num: u32,

    pub bump: u8,

    // Introducing a version as we are adding a new field bids_quote_lots
    pub version: u8,

    pub padding: [u8; 2],

    pub position: Position,

    pub open_orders: [OpenOrder; MAX_OPEN_ORDERS],
}

const_assert_eq!(
    size_of::<OpenOrdersAccount>(),
    size_of::<Pubkey>() * 2
        + 32
        + 32
        + 4
        + 1
        + 3
        + size_of::<Position>()
        + MAX_OPEN_ORDERS * size_of::<OpenOrder>()
);
const_assert_eq!(size_of::<OpenOrdersAccount>(), 1256);
const_assert_eq!(size_of::<OpenOrdersAccount>() % 8, 0);

impl OpenOrdersAccount {
    /// Number of bytes needed for the OpenOrdersAccount, including the discriminator
    pub fn space() -> usize {
        8 + size_of::<OpenOrdersAccount>()
    }

    pub fn name(&self) -> &str {
        std::str::from_utf8(&self.name)
            .unwrap()
            .trim_matches(char::from(0))
    }

    pub fn default_for_tests() -> Box<OpenOrdersAccount> {
        Box::new(OpenOrdersAccount {
            owner: Pubkey::default(),
            market: Pubkey::default(),
            name: [0; 32],
            delegate: NonZeroPubkeyOption::default(),
            account_num: 0,
            bump: 0,
            version: 1,
            padding: [0; 2],
            position: Position::default(),
            open_orders: [OpenOrder::default(); MAX_OPEN_ORDERS],
        })
    }

    pub fn is_owner_or_delegate(&self, ix_signer: Pubkey) -> bool {
        let delegate_option: Option<Pubkey> = Option::from(self.delegate);
        if let Some(delegate) = delegate_option {
            return self.owner == ix_signer || delegate == ix_signer;
        }
        self.owner == ix_signer
    }

    pub fn all_orders(&self) -> impl Iterator<Item = &OpenOrder> {
        self.open_orders.iter()
    }

    pub fn has_no_orders(&self) -> bool {
        self.open_orders.iter().count() == 0
    }

    pub fn all_orders_in_use(&self) -> impl Iterator<Item = &OpenOrder> {
        self.all_orders().filter(|oo| !oo.is_free())
    }

    pub fn next_order_slot(&self) -> Result<usize> {
        self.all_orders()
            .position(|&oo| oo.is_free())
            .ok_or_else(|| error!(OpenBookError::OpenOrdersFull))
    }

    pub fn find_order_with_client_order_id(&self, client_order_id: u64) -> Option<&OpenOrder> {
        self.all_orders_in_use()
            .find(|&oo| oo.client_id == client_order_id)
    }

    pub fn find_order_with_order_id(&self, order_id: u128) -> Option<&OpenOrder> {
        self.all_orders_in_use().find(|&oo| oo.id == order_id)
    }

    pub fn open_order_by_raw_index(&self, raw_index: usize) -> &OpenOrder {
        &self.open_orders[raw_index]
    }

    pub fn open_order_mut_by_raw_index(&mut self, raw_index: usize) -> &mut OpenOrder {
        &mut self.open_orders[raw_index]
    }

    pub fn execute_maker_atomic(
        &mut self,
        ctx: &Context<AtomicFinalize>,
        fill: &FillEvent,
        /*
        market: &mut Market,
        market_pda: &AccountInfo,
        maker_ata: AccountInfo,
        taker_ata: AccountInfo,
        token_program: &AccountInfo,
        market_base_vault: AccountInfo,
        market_quote_vault: AccountInfo, //TokenAccount
        program_id: Pubkey, */
        //token_program: &AccountInfo,
       // market_authority: &AccountInfo,
       // seeds: &[&[u8]],
    ) -> Result<()> {
        let market = &ctx.accounts.market.load_mut()?;
        let market_base_vault = &ctx.accounts.market_vault_base;
        let market_quote_vault = &ctx.accounts.market_vault_quote;
        let maker_ata = &ctx.accounts.maker_ata;
        let taker_ata = &ctx.accounts.taker_ata;
        let token_program = &ctx.accounts.token_program;
        let market_account_info = &ctx.accounts.market.to_account_info();
        let market_pda = market_account_info; //.key
        let program_id = ctx.program_id;
        let side = fill.taker_side().invert_side(); //i.e. maker side
        let quote_native = (fill.quantity * fill.price * market.quote_lot_size) as u64;
    
        // Calculate maker fees and rebates
        let is_self_trade = fill.maker == fill.taker;
        let (maker_fees, maker_rebate) = if is_self_trade {
            (0, 0)
        } else {
            (
                market.maker_fees_floor(quote_native),
                market.maker_rebate_floor(quote_native),
            )
        };
    
        // ... rest of your existing logic ...
    
        // JIT Transfers
        let transfer_amount = match side {
            Side::Bid => {
                // For a bid, calculate the amount in quote currency
                let quote_amount = (fill.quantity * fill.price * market.quote_lot_size) as u64;
                quote_amount // Assuming no free quote amount to subtract
            },
            Side::Ask => {
                // For an ask, calculate the amount in base currency
                let base_amount = (fill.quantity * market.base_lot_size) as u64;
                base_amount // Assuming no free base amount to subtract
            },
        };
    
        // Determine the from and to accounts for the transfer
        // REVIEW!
        let (from_account, to_account) = match side {
            Side::Ask => (taker_ata, market_base_vault),
            Side::Bid => (maker_ata, market_quote_vault),
        };
        // Construct the seeds for the market PDA
        let (market_pdas, bump_seed) = Pubkey::find_program_address(
            &[b"Market", market_pda.key().as_ref()],
            &program_id.key(),
        );
        // jit transfers
        let market_seed = b"Market";
        let bump_seed_arr = &[bump_seed];
        //let seeds = &[market_seed, market_pda.key().as_ref(), bump_seed_arr];
        //let seeds: &[&[u8]] = &[market_seed, market_pda.key().as_ref(), bump_seed_arr];
        let binding = market_pda.key();
        let seeds_slice: &[&[u8]] = &[market_seed, binding.as_ref(), bump_seed_arr];
        let seeds: &[&[&[u8]]] = &[seeds_slice];
        // Perform the transfer if the amount is greater than zero
        if transfer_amount > 0 {
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
            let cpi_accounts = Transfer {
                from: from_account.to_account_info(),
                to: to_account.to_account_info(),
                authority: market_pda.to_account_info(),
            };
            let cpi_context = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, seeds);
            anchor_spl::token::transfer(cpi_context, transfer_amount)?;
            Ok(())
        } 
        else {
            Ok(())
        }
    
        // ... rest of your logic for updating positions and emitting events ...
    
        //Ok(())
    }
    
        
    pub fn execute_maker(&mut self, market: &mut Market, fill: &FillEvent, token_program: &AccountInfo, program_id: Pubkey) {
        let is_self_trade = fill.maker == fill.taker;
        let user_pubkey = self.owner.key();
        //let user_ata_address = get_associated_token_address(&user_wallet_address, &token_mint_address);
        //let user_quote_account = ctx.accounts.user_quote_account.load_mut()?;
        //let program_id = program_id;

        let side = fill.taker_side().invert_side();
        let quote_native = (fill.quantity * fill.price * market.quote_lot_size) as u64;

        let (maker_fees, maker_rebate) = if is_self_trade {
            (0, 0)
        } else {
            (
                market.maker_fees_floor(quote_native),
                market.maker_rebate_floor(quote_native),
            )
        };

        let mut locked_maker_fees = maker_fees;
        let mut locked_amount_above_fill_price = 0;

        let locked_price = if fill.peg_limit != -1 && side == Side::Bid {
            let quote_at_lock_price =
                (fill.quantity * fill.peg_limit * market.quote_lot_size) as u64;
            let quote_to_free = quote_at_lock_price - quote_native;

            let fees_at_lock_price = market.maker_fees_floor(quote_at_lock_price);
            let fees_at_fill_price = maker_fees;
            let maker_fees_to_free = fees_at_lock_price - fees_at_fill_price;

            locked_maker_fees = fees_at_lock_price;
            locked_amount_above_fill_price = quote_to_free + maker_fees_to_free;
            fill.peg_limit
        } else {
            fill.price
        };

        {
            let pa = &mut self.position;

            match side {
                Side::Bid => {
                    pa.base_free_native += (fill.quantity * market.base_lot_size) as u64;
                    pa.quote_free_native += maker_rebate + locked_amount_above_fill_price;
                    pa.locked_maker_fees -= locked_maker_fees;
                }
                Side::Ask => {
                    pa.quote_free_native += quote_native + maker_rebate - maker_fees;
                }
            };

            pa.maker_volume += quote_native as u128;
            pa.referrer_rebates_available += maker_fees;
            market.referrer_rebates_accrued += maker_fees;
            market.maker_volume += quote_native as u128;
            market.fees_accrued += maker_fees as u128;

                    // Derive the market's PDA and bump seed
            let (market_pda, bump_seed) = Pubkey::find_program_address(
                &[b"Market", self.market.key().as_ref()],
                &program_id.key(),
            );
            // jit transfers
            let pa = &mut self.position;
            let user_quote_account = get_associated_token_address(&user_pubkey, &market.quote_mint);
            let user_base_account = get_associated_token_address(&user_pubkey, &market.base_mint);
            let transfer_amount = match fill.taker_side() {
                Side::Bid => {
                    // For a bid, calculate the amount in quote currency
                    let quote_amount = (fill.quantity * fill.price * market.quote_lot_size) as u64;
                    // Subtract any free quote amount already available
                    quote_amount.saturating_sub(pa.quote_free_native)
                    //User's ATA address for quotemint

                },
                Side::Ask => {
                    // For an ask, calculate the amount in base currency
                    let base_amount = (fill.quantity * market.base_lot_size) as u64;
                    // Subtract any free base amount already available
                    base_amount.saturating_sub(pa.base_free_native)

                },
            };

            // Determine the from and to accounts for the transfer
            let (from_account, to_account) = match fill.taker_side() {
                Side::Bid => (user_quote_account, market.market_quote_vault),
                Side::Ask => (user_base_account, market.market_base_vault),
            };

            // Construct the seeds for the market PDA
            let market_seed = b"Market";
            let bump_seed_arr = &[bump_seed];
            let seeds = &[market_seed, self.market.key().as_ref(), bump_seed_arr];

            // Perform the transfer if the amount is greater than zero
            if transfer_amount > 0 {
 
                /* 
                transfer_from_user(
                    transfer_amount,
                    &token_program,
                    &from_account,
                    &to_account,
                    &market_pda.into(), // Convert Pubkey to AccountInfo
                    seeds,
                ); */
            }

            if fill.maker_out() {
                self.remove_order(fill.maker_slot as usize, fill.quantity, locked_price);
            } else {
                match side {
                    Side::Bid => {
                        pa.bids_base_lots -= fill.quantity;
                        pa.bids_quote_lots -= fill.quantity * locked_price;
                    }
                    Side::Ask => pa.asks_base_lots -= fill.quantity,
                };
            }
        }

        emit!(FillLog {
            market: self.market,
            taker_side: fill.taker_side,
            maker_slot: fill.maker_slot,
            maker_out: fill.maker_out(),
            timestamp: fill.timestamp,
            seq_num: fill.seq_num,
            maker: fill.maker,
            maker_client_order_id: fill.maker_client_order_id,
            maker_fee: market.maker_fee,
            maker_timestamp: fill.maker_timestamp,
            taker: fill.taker,
            taker_client_order_id: fill.taker_client_order_id,
            taker_fee: market.taker_fee,
            price: fill.price,
            quantity: fill.quantity,
        });

        let pa = &self.position;
        emit!(OpenOrdersPositionLog {
            owner: self.owner,
            open_orders_account_num: self.account_num,
            market: self.market,
            bids_base_lots: pa.bids_base_lots,
            bids_quote_lots: pa.bids_quote_lots,
            asks_base_lots: pa.asks_base_lots,
            base_free_native: pa.base_free_native,
            quote_free_native: pa.quote_free_native,
            locked_maker_fees: pa.locked_maker_fees,
            referrer_rebates_available: pa.referrer_rebates_available,
            maker_volume: pa.maker_volume,
            taker_volume: pa.taker_volume
        })
    }

    /// Release funds and apply taker fees to the taker account. Account fees for referrer
    pub fn execute_taker(
        &mut self,
        market: &mut Market,
        taker_side: Side,
        base_native: u64,
        quote_native: u64,
        taker_fees: u64,
        referrer_amount: u64,
    ) {
        let pa = &mut self.position;
        match taker_side {
            Side::Bid => pa.base_free_native += base_native,
            Side::Ask => pa.quote_free_native += quote_native - taker_fees,
        };

        pa.taker_volume += quote_native as u128;
        pa.referrer_rebates_available += referrer_amount;
        market.referrer_rebates_accrued += referrer_amount;

        emit!(OpenOrdersPositionLog {
            owner: self.owner,
            open_orders_account_num: self.account_num,
            market: self.market,
            bids_base_lots: pa.bids_base_lots,
            bids_quote_lots: pa.bids_quote_lots,
            asks_base_lots: pa.asks_base_lots,
            base_free_native: pa.base_free_native,
            quote_free_native: pa.quote_free_native,
            locked_maker_fees: pa.locked_maker_fees,
            referrer_rebates_available: pa.referrer_rebates_available,
            maker_volume: pa.maker_volume,
            taker_volume: pa.taker_volume
        })
    }

    pub fn add_order(
        &mut self,
        side: Side,
        order_tree: BookSideOrderTree,
        order: &LeafNode,
        client_order_id: u64,
        locked_price: i64,
    ) {
        let position = &mut self.position;
        match side {
            Side::Bid => {
                position.bids_base_lots += order.quantity;
                position.bids_quote_lots += order.quantity * locked_price;
            }
            Side::Ask => position.asks_base_lots += order.quantity,
        };
        let slot = order.owner_slot as usize;

        let oo = self.open_order_mut_by_raw_index(slot);
        oo.is_free = false.into();
        oo.side_and_tree = SideAndOrderTree::new(side, order_tree).into();
        oo.id = order.key;
        oo.client_id = client_order_id;
        oo.locked_price = locked_price;
    }

    pub fn remove_order(&mut self, slot: usize, base_quantity: i64, locked_price: i64) {
        let oo = self.open_order_by_raw_index(slot);
        assert!(!oo.is_free());

        let order_side = oo.side_and_tree().side();
        let position = &mut self.position;

        // accounting
        match order_side {
            Side::Bid => {
                position.bids_base_lots -= base_quantity;
                position.bids_quote_lots -= base_quantity * locked_price;
            }
            Side::Ask => position.asks_base_lots -= base_quantity,
        }

        // release space
        *self.open_order_mut_by_raw_index(slot) = OpenOrder::default();
    }

    pub fn cancel_order(&mut self, slot: usize, base_quantity: i64, market: Market) {
        let oo = self.open_order_by_raw_index(slot);
        let price = oo.locked_price;
        let order_side = oo.side_and_tree().side();

        let base_quantity_native = (base_quantity * market.base_lot_size) as u64;
        let quote_quantity_native = (base_quantity * price * market.quote_lot_size) as u64;
        let fees = market.maker_fees_ceil(quote_quantity_native);

        let position = &mut self.position;
        match order_side {
            Side::Bid => {
                position.quote_free_native += quote_quantity_native + fees;
                position.locked_maker_fees -= fees;
            }
            Side::Ask => position.base_free_native += base_quantity_native,
        }

        self.remove_order(slot, base_quantity, price);
    }
}

#[zero_copy]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Position {
    /// Base lots in open bids
    pub bids_base_lots: i64,
    /// Base lots in open asks
    pub asks_base_lots: i64,

    pub base_free_native: u64,
    pub quote_free_native: u64,

    pub locked_maker_fees: u64,
    pub referrer_rebates_available: u64,
    /// Count of ixs when events are added to the heap
    /// To avoid this, send remaining accounts in order to process the events
    pub penalty_heap_count: u64,

    /// Cumulative maker volume in quote native units (display only)
    pub maker_volume: u128,
    /// Cumulative taker volume in quote native units (display only)
    pub taker_volume: u128,

    /// Quote lots in open bids
    pub bids_quote_lots: i64,

    #[derivative(Debug = "ignore")]
    pub reserved: [u8; 64],
}

const_assert_eq!(
    size_of::<Position>(),
    8 + 8 + 8 + 8 + 8 + 8 + 8 + 16 + 16 + 8 + 64
);
const_assert_eq!(size_of::<Position>(), 160);
const_assert_eq!(size_of::<Position>() % 8, 0);

impl Default for Position {
    fn default() -> Self {
        Self {
            bids_base_lots: 0,
            asks_base_lots: 0,
            base_free_native: 0,
            quote_free_native: 0,
            locked_maker_fees: 0,
            referrer_rebates_available: 0,
            penalty_heap_count: 0,
            maker_volume: 0,
            taker_volume: 0,
            bids_quote_lots: 0,
            reserved: [0; 64],
        }
    }
}

impl Position {
    /// Does the user have any orders on the book?
    ///
    /// Note that it's possible they were matched already: This only becomes
    /// false when the fill event is processed or the orders are cancelled.
    pub fn has_open_orders(&self) -> bool {
        self.asks_base_lots != 0 || self.bids_base_lots != 0
    }

    pub fn is_empty(&self, version: u8) -> bool {
        self.bids_base_lots == 0
            && self.asks_base_lots == 0
            && self.base_free_native == 0
            && self.quote_free_native == 0
            && self.locked_maker_fees == 0
            && self.referrer_rebates_available == 0
            && self.penalty_heap_count == 0
            // For version 0, bids_quote_lots was not properly tracked
            && (version == 0 || self.bids_quote_lots == 0)
    }
}

#[zero_copy]
#[derive(Debug)]
pub struct OpenOrder {
    pub id: u128,
    pub client_id: u64,
    /// Price at which user's assets were locked
    pub locked_price: i64,

    pub is_free: u8,
    pub side_and_tree: u8, // SideAndOrderTree -- enums aren't POD
    pub padding: [u8; 6],
}
const_assert_eq!(size_of::<OpenOrder>(), 16 + 8 + 8 + 1 + 1 + 6);
const_assert_eq!(size_of::<OpenOrder>(), 40);
const_assert_eq!(size_of::<OpenOrder>() % 8, 0);

impl Default for OpenOrder {
    fn default() -> Self {
        Self {
            is_free: true.into(),
            side_and_tree: SideAndOrderTree::BidFixed.into(),
            client_id: 0,
            locked_price: 0,
            id: 0,
            padding: [0; 6],
        }
    }
}

impl OpenOrder {
    pub fn is_free(&self) -> bool {
        self.is_free == u8::from(true)
    }

    pub fn side_and_tree(&self) -> SideAndOrderTree {
        SideAndOrderTree::try_from(self.side_and_tree).unwrap()
    }
}
