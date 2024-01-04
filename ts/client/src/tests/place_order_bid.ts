// import { Keypair, PublicKey } from '@solana/web3.js';
// import { OpenBookV2Client, Side } from '../client'; // Adjust the path as necessary
// import { BN } from '@coral-xyz/anchor';


import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';
// import * as spl from '@solana/spl-token';
import { OpenBookV2Client } from '../client'; // Adjust the path as necessary
import { BN, AnchorProvider, Wallet } from '@coral-xyz/anchor';
import {   checkOrCreateAssociatedTokenAccount, checkMintOfATA } from './utils2';
import { Side } from '../utils/utils';
import { airdropToken } from '../utils/airdrop';

// import { PlaceOrderType } from '../types/openbook_v2';


import * as fs from 'fs';

// const fs = require('fs');

// Constants:
/*
Market account: HiRQyetM9Axn1Wfs4LmyfVLbXJ5nASGy24cHMg2xoLC3
Bids account: 7uT4zMYj8bccV6o815WrLseDtM6LNnF7RgchzfdPK9o4
Asks account: BvwMRWMUoHEfBPL74ju1C2ELG8y7hjo8WfBTj9obZuPZ
Event heap account: GRaV5hgvuuRLXs5N6wW7DTyqYTMfgbrRNMAtFbowEMGx
Quote mint: Gm8JULsWJZwbMGPAUZm21mXqSPXv6TANuCvHkADXismA
Base mint: 8ktADAZBvgKVqB1y5ZhzEnYCu633ksxX6SzMJhh6owjF
Quote lot size: 1000000
Base lot size: 1000000000

*/

// async function placeOrder() {
  async function placeOrder(): Promise<void> {

  // Basic Config
  // const secretKey = JSON.parse(fs.readFileSync("/Users/dm/.config/solana/id.json"));
  const secretKey = JSON.parse(fs.readFileSync("/Users/dm/.config/solana/id.json", 'utf8'));

  const keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));
  const authority = keypair;
  // const payer = authority;
  

  // wrap authority in an anchor wallet
  const wallet = new Wallet(keypair);
  // const wallet = anchor.Wallet.local();

  const connection = new Connection("http://localhost:8899", "processed");
  // const connection = new Connection("https://api.devnet.solana.com", "processed");
  // provider setup
  // use default opts.
  const provider = new AnchorProvider(connection, wallet, {});
  // const provider = new OpenBookV2Client(connection);
  // const provider = /* your provider setup */;
  const ProgramId = new PublicKey("E6cNbXn2BNoMjXUg7biSTYhmTuyJWQtAnRX1fVPa7y5v");
  const client = new OpenBookV2Client(provider, ProgramId);

  // let market;
  const marketPublicKey = new PublicKey("HiRQyetM9Axn1Wfs4LmyfVLbXJ5nASGy24cHMg2xoLC3");
  const market = await client.deserializeMarketAccount(marketPublicKey);
  if (market == null) {
    throw new Error("Market is null");
}
  console.log("market setup done!");
  console.log("market: ", marketPublicKey.toString());
  console.log("client program id: ", client.programId.toString());

  const userPublicKey = keypair.publicKey;
  const openOrdersAccounts = await client.findOpenOrdersForMarket(userPublicKey, marketPublicKey);
  console.log("open orders accounts:", openOrdersAccounts);

  let openOrdersPublicKey;
  if (openOrdersAccounts.length === 0) {
    // User does not have an open orders account, create one
    const accountIndex = new BN(1); // Use an appropriate index
    const name = "FirstOO"; // Provide a name for the account
    openOrdersPublicKey = await client.createOpenOrders(
      authority, // Payer Keypair
      marketPublicKey,
      accountIndex,
      name,
      authority,
    );
    console.log("OO created!")
    console.log("public key: ", openOrdersPublicKey.toString());
  } else {
    // Use the existing open orders account
    console.log("OO exists!");
    openOrdersPublicKey = openOrdersAccounts[0];
    console.log("public key: ", openOrdersPublicKey.toString());  
  } 

  // const openOrdersPublicKey = /* Your Open Orders Public Key */;
  // check if ata exists, otherwise create it
  console.log("OO done!");

  // const userTokenAccount2 = await checkOrCreateAssociatedTokenAccount(provider, market.baseMint, userPublicKey);
  const userTokenAccount = new PublicKey(await checkOrCreateAssociatedTokenAccount(provider, market.quoteMint, userPublicKey));

  console.log("market quote mint: ", market.quoteMint.toString());
  console.log("quoteMint:", "Gm8JULsWJZwbMGPAUZm21mXqSPXv6TANuCvHkADXismA");
  // console.log("ata quote mint: ", userTokenAccount.toString());

  console.log("ATA done!");
  const userATAmint = await checkMintOfATA(connection, userTokenAccount);
  console.log("userTokenAccount: ", userATAmint.toString());

  // Airdrop Quote Token
  const airdropArgs = { receiverPk: userPublicKey,
    ownerKp: authority,
    connection: connection,
    mint: market.quoteMint,
    amount: 1000000000000, 
  }
  await airdropToken(airdropArgs);

  /* const orderArgs2 = {
    side: Side.Bid, // or { ask: {} } for an ask order
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
    clientOrderId: new BN(11),
    orderType: { limit: {} }, // 'limit' for a limit order, 'market' for a market order, etc.
    expiryTimestamp: new anchor.BN(Math.floor(Date.now() / 1000) + 3600), // Unix timestamp, e.g., 1 hour from now.
    selfTradeBehavior: { decrementTake: {} }, // Options might include 'decrementTake', 'cancelProvide', 'abortTransaction', etc.
    limit: 5,
    // selfTradeBehavior: /* self trade behavior */,
    // orderType: /* order type */,
    // limit: /* limit */,
  };

  console.log("config done!");
  console.log("openOrdersPublicKey: ", openOrdersPublicKey.toString());
  
  const openordersmaker = new PublicKey("YxFf7n5bBQYYsWBBxL8EqZ5qM9eDPoETaXjAh5SSCet");
//YxFf7n5bBQYYsWBBxL8EqZ5qM9eDPoETaXjAh5SSCet
  const [ix, signers] = await client.placeOrderIx(
    openOrdersPublicKey,
    marketPublicKey,
    market,
    userTokenAccount,
    null, // openOrdersAdmin
    orderArgs,
    [openordersmaker], // remainingAccounts
  );

  // Send transaction
  await client.sendAndConfirmTransaction([ix], {
    additionalSigners: signers,
  });

  console.log("Order placed successfully");
}

placeOrder().catch(console.error);
