import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { OpenBookV2Client } from '../client'; // Adjust the path as necessary
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { checkOrCreateAssociatedTokenAccount } from './utils2';
// import bn
import BN from 'bn.js';
// const fs = require('fs');
import * as fs from 'fs';

// async function finalizeEvents() {
async function finalizeEvents(): Promise<void> {

// Basic Config
  const secretKey = JSON.parse(fs.readFileSync("/Users/dm/.config/solana/id.json", "utf-8"));
  const keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));
  //const wallet = new Wallet(keypair);
  const connection = new Connection("http://localhost:8899", "processed");
  const programId = new PublicKey("E6cNbXn2BNoMjXUg7biSTYhmTuyJWQtAnRX1fVPa7y5v");

  const secretKeynew = JSON.parse(fs.readFileSync("/Users/dm/Documents/fermi_labs/m2/pro/Fermi-Pro/kp3/key.json", "utf-8"));
  const keypairnew = Keypair.fromSecretKey(new Uint8Array(secretKeynew));
  const authority = keypairnew;
  const wallet = new Wallet(keypairnew);
  const provider = new AnchorProvider(connection, wallet, {});
  const client = new OpenBookV2Client(provider, programId);

  // const payer = authority;
  console.log("keypairnew: ", keypairnew.publicKey.toString())

  

  // wrap authority in an anchor wallet
  //const wallet = new Wallet(keypairnew);


  // Market and Event Heap setup
  const marketPublicKey = new PublicKey("HiRQyetM9Axn1Wfs4LmyfVLbXJ5nASGy24cHMg2xoLC3");
  const eventHeapPublicKey = new PublicKey("GRaV5hgvuuRLXs5N6wW7DTyqYTMfgbrRNMAtFbowEMGx");
  const market = await client.deserializeMarketAccount(marketPublicKey);

  // Define the slots to consume (example: [0, 1, 2])
  const slotsToConsume = new BN(0);
  // [/* Array of slots to consume */];
  const makerpubkey = keypairnew.publicKey;
 
  // Additional accounts setup
  // const makerAtaPublicKey = /* Maker's ATA Public Key */;
  // const takerAtaPublicKey = /* Taker's ATA Public Key */;
  if (market != null) {
  const marketVaultBasePublicKey = market.marketBaseVault;
  /* Market's Base Vault Public Key */
  const marketVaultQuotePublicKey = market.marketQuoteVault;
  const makerAtaPublicKey = new PublicKey(await checkOrCreateAssociatedTokenAccount(provider, market.baseMint, makerpubkey));
  const takerAtaPublicKey = new PublicKey(await checkOrCreateAssociatedTokenAccount(provider, market.quoteMint, makerpubkey));
  //const makerOpenOrder = await client.deserializeOpenOrderAccount(makerpubkey);
  const makerOpenOrder = await client.findAllOpenOrders(makerpubkey);
  const makeropenorderfirst = makerOpenOrder[0];
  console.log("makeropenorderfirst: ", makeropenorderfirst.toBase58());
  /* Market's Quote Vault Public Key */
  // const tokenProgramPublicKey = 
  /* Token Program Public Key */
  //msg!("finalizing events")
  // Create the instruction for finalizing events
  const [ix, signers] = await client.createFinalizeEventsInstruction(
    marketPublicKey,
    market,
    eventHeapPublicKey,
    makerAtaPublicKey,
    takerAtaPublicKey,
    marketVaultBasePublicKey,
    marketVaultQuotePublicKey,
    makeropenorderfirst,
    //makerpubkey,
    //tokenProgramPublicKey,
    slotsToConsume
  );

  // Send transaction
  await client.sendAndConfirmTransaction([ix], {
    additionalSigners: signers,
  });

  console.log("Events finalized successfully");
}
}
finalizeEvents().catch(console.error);
