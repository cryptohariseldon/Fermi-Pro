import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';
// import * as spl from '@solana/spl-token';
import { OpenBookV2Client } from '../client'; // Adjust the path as necessary
import { BN, AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { createMint } from './utils2';


import * as fs from 'fs';

// const fs = require('fs');


async function initializeMarket(): Promise<void> {
  // Initialize connection and client
  // connection to localhost at 8899
  // const {Keypair} = require("@solana/web3.js");
  const secretKey = JSON.parse(fs.readFileSync("/Users/dm/.config/solana/id.json", "utf-8"));
  const keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));
  const authority = keypair;
  const payer = authority;

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

  const coinMint = anchor.web3.Keypair.generate();
  const pcMint = anchor.web3.Keypair.generate();

  await createMint(provider, coinMint, 9);
  await createMint(provider, pcMint, 6);
  


  // const payer = Keypair.generate(); // This should be your funded account keypair

  const quoteMint = new PublicKey(coinMint.publicKey.toBase58());
  const baseMint = new PublicKey(pcMint.publicKey.toBase58());

  // Define market parameters
  // quote lot 10e6
  // base lot 10e9
  const quoteLotSize = new BN(1000000);
  const baseLotSize = new BN(1000000000);
  const makerFee = new BN(0);
  const takerFee = new BN(0);
  const timeExpiry = new BN(0);
  /*
  // Create market
  //const [ixs, signers]
  const [[bidIx, askIx, eventHeapIx, ix,], [market, bidsKeypair, askKeypair, eventHeapKeypair]] = await client.createMarketIx(
    payer.publicKey,
    "Market Name",
    quoteMint,
    baseMint,
    quoteLotSize,
    baseLotSize,
    makerFee,
    takerFee,
    timeExpiry,
    null, // oracleA
    null, // oracleB
    null, // openOrdersAdmin
    null, // consumeEventsAdmin
    null, // closeMarketAdmin
  );
  
  const [ixs, signers] = await client.createMarketIx(
    payer.publicKey,
    "Market Name",
    quoteMint,
    baseMint,
    quoteLotSize,
    baseLotSize,
    makerFee,
    takerFee,
    timeExpiry,
    null, // oracleA
    null, // oracleB
    null, // openOrdersAdmin
    null, // consumeEventsAdmin
    null, // closeMarketAdmin
  );

  // Send transaction
  await client.sendAndConfirmTransaction(ixs, {
    additionalSigners: [payer, ...signers],
  });

  console.log("Market initialized successfully");
  console.log("Market account:", market.publicKey.toBase58());
  console.log("Bids account:", bidsKeypair.publicKey.toBase58());
  console.log("Asks account:", askKeypair.publicKey.toBase58());
  console.log("Event heap account:", eventHeapKeypair.publicKey.toBase58());
  //console.log("Market authority:", market.authority.toBase58());
  console.log("Quote mint:", quoteMint.toBase58());
  console.log("Base mint:", baseMint.toBase58());
  console.log("Quote lot size:", quoteLotSize.toString());
  console.log("Base lot size:", baseLotSize.toString());
  //console.log("Maker fee:", makerFee.toString());
}

initializeMarket().catch(console.error); */

const [[bidIx, askIx, eventHeapIx, ix], [market, bidsKeypair, askKeypair, eventHeapKeypair]] = await client.createMarketIx(
  payer.publicKey,
  "Market Name",
  quoteMint,
  baseMint,
  quoteLotSize,
  baseLotSize,
  makerFee,
  takerFee,
  timeExpiry,
  null, // oracleA
  null, // oracleB
  null, // openOrdersAdmin
  null, // consumeEventsAdmin
  null, // closeMarketAdmin
);

// Send transaction
await client.sendAndConfirmTransaction([bidIx, askIx, eventHeapIx, ix], {
  additionalSigners: [payer, market, bidsKeypair, askKeypair, eventHeapKeypair],
});

console.log("Market initialized successfully");
console.log("Market account:", market.publicKey.toBase58());
console.log("Bids account:", bidsKeypair.publicKey.toBase58());
console.log("Asks account:", askKeypair.publicKey.toBase58());
console.log("Event heap account:", eventHeapKeypair.publicKey.toBase58());
// console.log("Market authority:", market.authority.toBase58());
console.log("Quote mint:", quoteMint.toBase58());
console.log("Base mint:", baseMint.toBase58());
console.log("Quote lot size:", quoteLotSize.toString());
console.log("Base lot size:", baseLotSize.toString());
// console.log("Maker fee:", makerFee.toString());
}

initializeMarket().catch(console.error);
