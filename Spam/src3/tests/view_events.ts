// import { Keypair, PublicKey } from '@solana/web3.js';
// import { OpenBookV2Client, Side } from '../client'; // Adjust the path as necessary
// import { BN } from '@coral-xyz/anchor';

import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';
// import * as spl from '@solana/spl-token';
import { OpenBookV2Client } from '../client'; // Adjust the path as necessary
import { BN, AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { checkOrCreateAssociatedTokenAccount, checkMintOfATA } from './utils2';
import { Side } from '../utils/utils';
import { airdropToken } from '../utils/airdrop';
// import {deserializeEventHeapAccount } from '../client';

// import { PlaceOrderType } from '../types/openbook_v2';

import * as fs from 'fs';
// const fs = require('fs');

// Constants:
/*
Market account: ATPpcGQEWoh1fGuuY4AkHHGSD3WdHLUXg3XVseQo3K98
Bids account: H3BtfqqjdtDiRhbNC8ak3bAG5u1Rsr5bU2tHwoxxN4h9
Asks account: HcgSFXnFmYrJ9UtPboL8RSKY2a8kXaudWYxWzGNkiKwm
Event heap account: 8E4MNizP4pkX3Kp97qdKKDuf3Q1Zaa6d7p1Eu8egyNhz
Quote mint: BPm2ocHacN6oYpWGz67qztvAwBBBGeCjVtCddLEzh2Y6
Base mint: 8Ny99DoJwwb8v8FdzRWvtRPJvjqzLNmdZyAA5GGTWjgi
Quote lot size: 1000000
Base lot size: 1000000000
*/

async function placeOrder(): Promise<void> {
  // Your implementation here
}

// Basic Config
const secretKey = JSON.parse(
  fs.readFileSync('/Users/dm/.config/solana/id.json', 'utf-8'),
);
const keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));
// const authority = keypair;
// const payer = authority;

const secretKeynew = JSON.parse(
  fs.readFileSync(
    '/Users/dm/Documents/fermi_labs/m2/pro/Fermi-Pro/kp3/key.json',
    'utf-8',
  ),
);
const keypairnew = Keypair.fromSecretKey(new Uint8Array(secretKeynew));
const authority = keypairnew;
const payer = authority;

// wrap authority in an anchor wallet
const wallet = new Wallet(keypair);
// const wallet = anchor.Wallet.local();

const connection = new Connection('http://localhost:8899', 'processed');
// const connection = new Connection("https://api.devnet.solana.com", "processed");
// provider setup
// use default opts.
const provider = new AnchorProvider(connection, wallet, {});
// const provider = new OpenBookV2Client(connection);
// const provider = /* your provider setup */;
const ProgramId = new PublicKey('E6cNbXn2BNoMjXUg7biSTYhmTuyJWQtAnRX1fVPa7y5v');
const client = new OpenBookV2Client(provider, ProgramId);

// market;
const marketPublicKey = new PublicKey(
  'ATPpcGQEWoh1fGuuY4AkHHGSD3WdHLUXg3XVseQo3K98',
);
const market = await client.deserializeMarketAccount(marketPublicKey)!;
if (market == null) {
  throw new Error('Market is null');
}
console.log('market setup done!');
console.log('market: ', marketPublicKey.toString());
console.log('client program id: ', client.programId.toString());

const userPublicKey = keypair.publicKey;
const openOrdersAccounts = await client.findOpenOrdersForMarket(
  userPublicKey,
  marketPublicKey,
);
console.log('open orders accounts:', openOrdersAccounts);

let openOrdersPublicKey;
if (openOrdersAccounts.length === 0) {
  // User does not have an open orders account, create one
  const accountIndex = new BN(1); // Use an appropriate index
  const name = 'FirstOO'; // Provide a name for the account
  openOrdersPublicKey = await client.createOpenOrders(
    authority, // Payer Keypair
    marketPublicKey,
    accountIndex,
    name,
    authority,
  );
  console.log('OO created!');
  console.log('public key: ', openOrdersPublicKey.toString());
} else {
  // Use the existing open orders account
  console.log('OO exists!');
  openOrdersPublicKey = openOrdersAccounts[0];
  console.log('public key: ', openOrdersPublicKey.toString());
}

// const openOrdersPublicKey = /* Your Open Orders Public Key */;
// check if ata exists, otherwise create it
console.log('OO done!');

// const userTokenAccount2 = await checkOrCreateAssociatedTokenAccount(provider, market.baseMint, userPublicKey);
const userTokenAccount = await checkOrCreateAssociatedTokenAccount(
  provider,
  market.quoteMint,
  userPublicKey,
);

console.log('market quote mint: ', market.quoteMint.toString());
console.log('quoteMint:', 'BPm2ocHacN6oYpWGz67qztvAwBBBGeCjVtCddLEzh2Y6');
// console.log("ata quote mint: ", userTokenAccount.toString());

console.log('ATA done!');
const userATAmint = await checkMintOfATA(connection, userTokenAccount);
console.log('userTokenAccount: ', userATAmint.toString());

// Airdrop Quote Token
const airdropArgs = {
  receiverPk: userPublicKey,
  ownerKp: authority,
  connection: connection,
  mint: market.quoteMint,
  amount: 1000000000000,
};
await airdropToken(airdropArgs);

/* const orderArgs2 = {
    side: Side.Ask, // or { ask: {} } for an ask order
    priceLots: new BN(1000), // Replace with the appropriate value for price in lots
    maxBaseLots: new BN(1), // Replace with the appropriate value for max base quantity in lots
    maxQuoteLotsIncludingFees: new BN(1000), // Replace with the appropriate value for max quote quantity in lots, including fees
    clientOrderId: new BN(10),
  } */
const orderArgs = {
  side: Side.Bid, // or Side.Ask
  // side: 'bid',
  priceLots: new BN(1000), // Replace with the appropriate value for price in lots
  maxBaseLots: new BN(1), // Replace with the appropriate value for max base quantity in lots
  maxQuoteLotsIncludingFees: new BN(1000), // Replace with the appropriate value for max quote quantity in lots, including fees
  clientOrderId: new BN(10),
  orderType: { limit: {} }, // 'limit' for a limit order, 'market' for a market order, etc.
  expiryTimestamp: new anchor.BN(Math.floor(Date.now() / 1000) + 3600), // Unix timestamp, e.g., 1 hour from now.
  selfTradeBehavior: { decrementTake: {} }, // Options might include 'decrementTake', 'cancelProvide', 'abortTransaction', etc.
  limit: 5,
  // selfTradeBehavior: /* self trade behavior */,
  // orderType: /* order type */,
  // limit: /* limit */,
};

console.log('config done!');
console.log('openOrdersPublicKey: ', openOrdersPublicKey.toString());

/* const [ix, signers] = await client.placeOrderIx(
    openOrdersPublicKey,
    marketPublicKey,
    market,
    userTokenAccount,
    null, // openOrdersAdmin
    orderArgs,
    [], // remainingAccounts
  ); */

const eventQ = await client.deserializeEventHeapAccount(
  new PublicKey('8E4MNizP4pkX3Kp97qdKKDuf3Q1Zaa6d7p1Eu8egyNhz'),
);
console.log('eventQ: ', eventQ);
if (eventQ !== null) {
  const event1 = eventQ.nodes[0];
  // const event1 = eventQ.nodes[0].event;
  console.log('event1: ', event1);
}
const fillevent = await client.getAccountsToConsume(market);
console.log('fillevent:', fillevent);
/*
  // Send transaction
  await client.sendAndConfirmTransaction([ix], {
    additionalSigners: signers,
  });

  console.log("Order placed successfully");
} */

placeOrder().catch(console.error);
