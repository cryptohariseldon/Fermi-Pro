use super::*;

#[tokio::test]
async fn insufficient_funds() -> Result<(), TransportError> {
    let base_lot_size = 100;
    let quote_lot_size = 10;

    let TestInitialize {
        context,
        owner,
        owner_token_0,
        owner_token_1,
        account_1,
        account_2,
        market,
        market_base_vault,
        market_quote_vault,
        ..
    } = TestContext::new_with_market(TestNewMarketInitialize {
        base_lot_size,
        quote_lot_size,
        ..TestNewMarketInitialize::default()
    })
    .await?;

    let solana = &context.solana.clone();

    // there's an ask on the book
    send_tx(
        solana,
        PlaceOrderInstruction {
            open_orders_account: account_2,
            open_orders_admin: None,
            market,
            signer: owner,
            user_token_account: owner_token_0,
            market_vault: market_base_vault,
            side: Side::Ask,
            price_lots: 1,
            max_base_lots: 10,
            max_quote_lots_including_fees: i64::MAX / 1_000_000,
            client_order_id: 0,
            expiry_timestamp: 0,
            order_type: PlaceOrderType::Limit,
            self_trade_behavior: SelfTradeBehavior::default(),
            remainings: vec![],
        },
    )
    .await
    .unwrap();

    solana.set_account_balance(owner_token_0, 2_500).await;
    solana.set_account_balance(owner_token_1, 110).await;

    // some lamports are already deposited
    send_tx(
        solana,
        DepositInstruction {
            owner,
            market,
            open_orders_account: account_1,
            market_base_vault,
            market_quote_vault,
            user_base_account: owner_token_0,
            user_quote_account: owner_token_1,
            base_amount: 1_200,
            quote_amount: 0,
        },
    )
    .await
    .unwrap();

    // note that a priori, we only have enough lamports to place 2.5 Ask. But as the bid will be
    // filled & the taker executed immediately, we will have 10 extra base lots available
    let place_orders = (0..5)
        .map(|i| {
            if i == 1 {
                openbook_v2::PlaceOrderArgs {
                    side: Side::Bid,
                    price_lots: 1,
                    max_base_lots: 10,
                    max_quote_lots_including_fees: i64::MAX / 1_000_000,
                    client_order_id: 0,
                    order_type: PlaceOrderType::Limit,
                    expiry_timestamp: 0,
                    self_trade_behavior: SelfTradeBehavior::default(),
                    limit: 10,
                }
            } else {
                openbook_v2::PlaceOrderArgs {
                    side: Side::Ask,
                    price_lots: 1,
                    max_base_lots: 10,
                    max_quote_lots_including_fees: i64::MAX / 1_000_000,
                    client_order_id: 0,
                    order_type: PlaceOrderType::Limit,
                    expiry_timestamp: 0,
                    self_trade_behavior: SelfTradeBehavior::default(),
                    limit: 10,
                }
            }
        })
        .collect::<Vec<_>>();

    send_tx(
        solana,
        CancelAndPlaceOrdersInstruction {
            open_orders_account: account_1,
            open_orders_admin: None,
            market,
            signer: owner,
            user_base_account: owner_token_0,
            user_quote_account: owner_token_1,
            cancel_client_orders_ids: vec![],
            place_orders,
        },
    )
    .await
    .unwrap();

    let position = solana
        .get_account::<OpenOrdersAccount>(account_1)
        .await
        .position;

    assert_eq!(position.asks_base_lots, 35);
    assert_eq!(position.bids_base_lots, 0);

    assert_eq!(position.base_free_native, 0);
    assert_eq!(position.quote_free_native, 0);

    assert_eq!(position.referrer_rebates_available, 1);
    assert_eq!(solana.token_account_balance(owner_token_1).await, 9);
    assert_eq!(solana.token_account_balance(owner_token_0).await, 0);

    Ok(())
}
