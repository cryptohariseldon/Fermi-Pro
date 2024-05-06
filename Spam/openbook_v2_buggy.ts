export interface openbook_v2 {
  version: '0.1.0';
  name: 'openbook_v2';
  instructions: [
    {
      name: 'createMarket';
      docs: ['Create a [`Market`](crate::state::Market) for a given token pair.'];
      accounts: [
        {
          name: 'market';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'marketAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
          docs: ['Accounts are initialized by client,', 'anchor discriminator is set first when ix exits,'];
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'marketBaseVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketQuoteVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'baseMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'quoteMint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'associatedTokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'oracleA';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'oracleB';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'collectFeeAdmin';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'openOrdersAdmin';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'consumeEventsAdmin';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'closeMarketAdmin';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'eventAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'program';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'name';
          type: 'string';
        },
        {
          name: 'oracleConfig';
          type: 'OracleConfigParams';
        },
        {
          name: 'quoteLotSize';
          type: 'bigint';
        },
        {
          name: 'baseLotSize';
          type: 'bigint';
        },
        {
          name: 'makerFee';
          type: 'bigint';
        },
        {
          name: 'takerFee';
          type: 'bigint';
        },
        {
          name: 'timeExpiry';
          type: 'bigint';
        },
      ];
    },
    {
      name: 'closeMarket';
      docs: ['Close a [`Market`](crate::state::Market) (only', '[`close_market_admin`](crate::state::Market::close_market_admin)).'];
      accounts: [
        {
          name: 'closeMarketAdmin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'solDestination';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'createOpenOrdersIndexer';
      docs: ['Create an [`OpenOrdersIndexer`](crate::state::OpenOrdersIndexer) account.'];
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersIndexer';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'closeOpenOrdersIndexer';
      docs: ['Close an [`OpenOrdersIndexer`](crate::state::OpenOrdersIndexer) account.'];
      accounts: [
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersIndexer';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'solDestination';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'createOpenOrdersAccount';
      docs: ['Create an [`OpenOrdersAccount`](crate::state::OpenOrdersAccount).'];
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'delegateAccount';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'openOrdersIndexer';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'name';
          type: 'string';
        },
      ];
    },
    {
      name: 'closeOpenOrdersAccount';
      docs: ['Close an [`OpenOrdersAccount`](crate::state::OpenOrdersAccount).'];
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersIndexer';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'solDestination';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'placeOrder';
      docs: ['Place an order.', '', 'Different types of orders have different effects on the order book,', 'as described in [`PlaceOrderType`](crate::state::PlaceOrderType).', '', '`price_lots` refers to the price in lots: the number of quote lots', 'per base lot. It is ignored for `PlaceOrderType::Market` orders.', '', '`expiry_timestamp` is a unix timestamp for when this order should', 'expire. If 0 is passed in, the order will never expire. If the time', 'is in the past, the instruction is skipped. Timestamps in the future', 'are reduced to now + 65,535s.', '', '`limit` determines the maximum number of orders from the book to fill,', 'and can be used to limit CU spent. When the limit is reached, processing', 'stops and the instruction succeeds.'];
      accounts: [
        {
          name: 'signer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAdmin';
          isMut: false;
          isSigner: true;
          isOptional: true;
        },
        {
          name: 'userTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'oracleA';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'oracleB';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'args';
          type: 'PlaceOrderArgs';
        },
      ];
    },
    {
      name: 'editOrder';
      docs: ['Edit an order.'];
      accounts: [
        {
          name: 'signer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAdmin';
          isMut: false;
          isSigner: true;
          isOptional: true;
        },
        {
          name: 'userTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'oracleA';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'oracleB';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'clientOrderId';
          type: 'any';
        },
        {
          name: 'expectedCancelSize';
          type: 'bigint';
        },
        {
          name: 'placeOrder';
          type: 'PlaceOrderArgs';
        },
      ];
    },
    {
      name: 'editOrderPegged';
      docs: ['Edit an order pegged.'];
      accounts: [
        {
          name: 'signer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAdmin';
          isMut: false;
          isSigner: true;
          isOptional: true;
        },
        {
          name: 'userTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'oracleA';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'oracleB';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'clientOrderId';
          type: 'any';
        },
        {
          name: 'expectedCancelSize';
          type: 'bigint';
        },
        {
          name: 'placeOrder';
          type: 'PlaceOrderPeggedArgs';
        },
      ];
    },
    {
      name: 'cancelAndPlaceOrders';
      docs: ['Cancel orders and place multiple orders.'];
      accounts: [
        {
          name: 'signer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAdmin';
          isMut: false;
          isSigner: true;
          isOptional: true;
        },
        {
          name: 'userQuoteAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userBaseAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketQuoteVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketBaseVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'oracleA';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'oracleB';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'cancelClientOrdersIds';
          type: 'any';
        },
        {
          name: 'placeOrders';
          type: 'any';
        },
      ];
    },
    {
      name: 'placeOrderPegged';
      docs: ['Place an oracle-peg order.'];
      accounts: [
        {
          name: 'signer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAdmin';
          isMut: false;
          isSigner: true;
          isOptional: true;
        },
        {
          name: 'userTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'oracleA';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'oracleB';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'args';
          type: 'PlaceOrderPeggedArgs';
        },
      ];
    },
    {
      name: 'placeTakeOrder';
      docs: ['Place an order that shall take existing liquidity off of the book, not', 'add a new order off the book.', '', 'This type of order allows for instant token settlement for the taker.'];
      accounts: [
        {
          name: 'signer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'penaltyPayer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketBaseVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketQuoteVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userBaseAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userQuoteAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'referrerAccount';
          isMut: true;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'oracleA';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'oracleB';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'openOrdersAdmin';
          isMut: false;
          isSigner: true;
          isOptional: true;
        },
      ];
      args: [
        {
          name: 'args';
          type: 'PlaceTakeOrderArgs';
        },
      ];
    },
    {
      name: 'consumeEvents';
      docs: ['Process up to `limit` [events](crate::state::AnyEvent).', '', 'When a user places a 'take' order, they do not know beforehand which', 'market maker will have placed the 'make' order that they get executed', 'against. This prevents them from passing in a market maker's', '[`OpenOrdersAccount`](crate::state::OpenOrdersAccount), which is needed', 'to credit/debit the relevant tokens to/from the maker. As such, Openbook', 'uses a 'crank' system, where `place_order` only emits events, and', '`consume_events` handles token settlement.', '', 'Currently, there are two types of events: [`FillEvent`](crate::state::FillEvent)s', 'and [`OutEvent`](crate::state::OutEvent)s.', '', 'A `FillEvent` is emitted when an order is filled, and it is handled by', 'debiting whatever the taker is selling from the taker and crediting', 'it to the maker, and debiting whatever the taker is buying from the', 'maker and crediting it to the taker. Note that *no tokens are moved*,', 'these are just debits and credits to each party's [`Position`](crate::state::Position).', '', 'An `OutEvent` is emitted when a limit order needs to be removed from', 'the book during a `place_order` invocation, and it is handled by', 'crediting whatever the maker would have sold (quote token in a bid,', 'base token in an ask) back to the maker.'];
      accounts: [
        {
          name: 'consumeEventsAdmin';
          isMut: false;
          isSigner: true;
          isOptional: true;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'limit';
          type: 'any';
        },
      ];
    },
    {
      name: 'consumeGivenEvents';
      docs: ['Process the [events](crate::state::AnyEvent) at the given positions.'];
      accounts: [
        {
          name: 'consumeEventsAdmin';
          isMut: false;
          isSigner: true;
          isOptional: true;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'eventHeap';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'slots';
          type: 'any';
        },
      ];
    },
    {
      name: 'cancelOrder';
      docs: ['Cancel an order by its `order_id`.', '', 'Note that this doesn't emit an [`OutEvent`](crate::state::OutEvent) because a', 'maker knows that they will be passing in their own [`OpenOrdersAccount`](crate::state::OpenOrdersAccount).'];
      accounts: [
        {
          name: 'signer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'orderId';
          type: 'any';
        },
      ];
    },
    {
      name: 'cancelOrderByClientOrderId';
      docs: ['Cancel an order by its `client_order_id`.', '', 'Note that this doesn't emit an [`OutEvent`](crate::state::OutEvent) because a', 'maker knows that they will be passing in their own [`OpenOrdersAccount`](crate::state::OpenOrdersAccount).'];
      accounts: [
        {
          name: 'signer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'clientOrderId';
          type: 'any';
        },
      ];
    },
    {
      name: 'cancelAllOrders';
      docs: ['Cancel up to `limit` orders, optionally filtering by side'];
      accounts: [
        {
          name: 'signer';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'sideOption';
          type: 'any';
        },
        {
          name: 'limit';
          type: 'any';
        },
      ];
    },
    {
      name: 'deposit';
      docs: ['Deposit a certain amount of `base` and `quote` lamports into one's', '[`Position`](crate::state::Position).', '', 'Makers might wish to `deposit`, rather than have actual tokens moved for', 'each trade, in order to reduce CUs.'];
      accounts: [
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'userBaseAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userQuoteAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketBaseVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketQuoteVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'baseAmount';
          type: 'any';
        },
        {
          name: 'quoteAmount';
          type: 'any';
        },
      ];
    },
    {
      name: 'refill';
      docs: ['Refill a certain amount of `base` and `quote` lamports. The amount being passed is the', 'total lamports that the [`Position`](crate::state::Position) will have.', '', 'Makers might wish to `refill`, rather than have actual tokens moved for', 'each trade, in order to reduce CUs.'];
      accounts: [
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'userBaseAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userQuoteAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketBaseVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketQuoteVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'baseAmount';
          type: 'any';
        },
        {
          name: 'quoteAmount';
          type: 'any';
        },
      ];
    },
    {
      name: 'settleFunds';
      docs: ['Withdraw any available tokens.'];
      accounts: [
        {
          name: 'owner';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'penaltyPayer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'marketBaseVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketQuoteVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userBaseAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userQuoteAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'referrerAccount';
          isMut: true;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'settleFundsExpired';
      docs: ['Withdraw any available tokens when the market is expired (only', '[`close_market_admin`](crate::state::Market::close_market_admin)).'];
      accounts: [
        {
          name: 'closeMarketAdmin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'owner';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'penaltyPayer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'marketBaseVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketQuoteVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userBaseAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'userQuoteAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'referrerAccount';
          isMut: true;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'sweepFees';
      docs: ['Sweep fees, as a [`Market`](crate::state::Market)'s admin.'];
      accounts: [
        {
          name: 'collectFeeAdmin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'marketAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'marketQuoteVault';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenReceiverAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'setDelegate';
      docs: ['Update the [`delegate`](crate::state::OpenOrdersAccount::delegate) of an open orders account.'];
      accounts: [
        {
          name: 'owner';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'delegateAccount';
          isMut: false;
          isSigner: false;
          isOptional: true;
        },
      ];
      args: [
      ];
    },
    {
      name: 'setMarketExpired';
      docs: ['Set market to expired before pruning orders and closing the market (only', '[`close_market_admin`](crate::state::Market::close_market_admin)).'];
      accounts: [
        {
          name: 'closeMarketAdmin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'market';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'pruneOrders';
      docs: ['Remove orders from the book when the market is expired (only', '[`close_market_admin`](crate::state::Market::close_market_admin)).'];
      accounts: [
        {
          name: 'closeMarketAdmin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'openOrdersAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'market';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bids';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'asks';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'limit';
          type: 'any';
        },
      ];
    },
    {
      name: 'stubOracleCreate';
      docs: [];
      accounts: [
        {
          name: 'payer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'oracle';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'mint';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'price';
          type: 'any';
        },
      ];
    },
    {
      name: 'stubOracleClose';
      docs: [];
      accounts: [
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'oracle';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'solDestination';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
      ];
      args: [
      ];
    },
    {
      name: 'stubOracleSet';
      docs: [];
      accounts: [
        {
          name: 'owner';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'oracle';
          isMut: true;
          isSigner: false;
        },
      ];
      args: [
        {
          name: 'price';
          type: 'any';
        },
      ];
    },
  ];
}
