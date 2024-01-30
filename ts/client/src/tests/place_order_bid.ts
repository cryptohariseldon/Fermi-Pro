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
OLD
Market account: HKH41bEDxDSM9bZ3BzsfQt6VewFqaTMjSqpcTn1tWyB9
Bids account: B7ZuBt8hvEsqE5f7cfFoGGAxN2ZRPxeu51Rc7VGHpWRZ
Asks account: DWuE5fzzQfjcZErwWk7F191UyBeVtn81oHuSTrvpJYrX
Event heap account: 8JJCikPteizQvLhLzB6K1k46FbAgBTaquByFhSHhZcu7
Quote mint: 89Qranxmv2sr9q4is7eyPWHCby8MW1KabrLxJNc8wnJR
Base mint: 44mzg7c4qe3q8Cgw2zEQvSqXfoGra8S9856oQ3Z7Yep8


NEW
Market account: 7dip6pqWaASaqauWmYxUozoda4bJYL2BuAhnEFoSHvG4
Bids account: HiGwFTSk39cN1mK38Q8f7ga27iMiKJ3rxz1KnijVU2gP
Asks account: 5BjxQd3waCeMwe5cYrz8ChYp64wgwba85YebxVaGj9YM
Event heap account: 5QHKFzwJ6CxQpWcoeUqvXi85ijrNwS24T5aPWGum7owM
Quote mint: 7eFxBUvsUC69sHgqkKNz5TdvcZML45EaUc4sKzj4J3fH
Base mint: 969s7LFrDDqeJF7VSjb78H7BH3TfyUoSZc3oazwSbjHw
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

  //const connection = new Connection("http://localhost:8899", "processed");
  const connection = new Connection("https://api.devnet.solana.com", "processed");
  // provider setup
  // use default opts.
  const provider = new AnchorProvider(connection, wallet, {});
  // const provider = new OpenBookV2Client(connection);
  // const provider = /* your provider setup */;
  const ProgramId = new PublicKey("E6cNbXn2BNoMjXUg7biSTYhmTuyJWQtAnRX1fVPa7y5v");
  const client = new OpenBookV2Client(provider, ProgramId);

  // let market;
  const marketPublicKey = new PublicKey("4HosRSq8RgDv4oZeKJYU8gTe1kns5UMumE345iKQVaLo");
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
  const [marketAuthorityPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from('Market'), marketPublicKey.toBuffer()],
    ProgramId,
  );
  const openordersmaker = new PublicKey(openOrdersPublicKey.toString());  
//YxFf7n5bBQYYsWBBxL8EqZ5qM9eDPoETaXjAh5SSCet
  const [ix, signers] = await client.placeOrderIx(
    openOrdersPublicKey,
    marketPublicKey,
    market,
    marketAuthorityPDA,
    userTokenAccount,
    null, // openOrdersAdmin
    orderArgs,
    [openordersmaker], // remainingAccounts
  );
//CHECK LAST ARG!
  // Send transaction
  await client.sendAndConfirmTransaction([ix], {
    additionalSigners: signers,
  });

  console.log("Order placed successfully");
}

placeOrder().catch(console.error);
