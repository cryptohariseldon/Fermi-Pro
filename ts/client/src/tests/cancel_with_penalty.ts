import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { OpenBookV2Client } from '../client'; // Adjust the path as necessary
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { checkOrCreateAssociatedTokenAccount } from './utils2';
// import bn
import BN from 'bn.js';
// const fs = require('fs');
import * as fs from 'fs';
import { Side } from '../utils/utils';


// async function finalizeEvents() {
async function finalizeEvents(): Promise<void> {

// Basic Config
  const secretKey = JSON.parse(fs.readFileSync("/Users/dm/.config/solana/id.json", "utf-8"));
  const keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));
  //const wallet = new Wallet(keypair);
  const connection = new Connection("http://127.0.0.1:8899", "processed");
  //devnet
  //const connection = new Connection("https://api.devnet.solana.com", "processed");
  const programId = new PublicKey("E6cNbXn2BNoMjXUg7biSTYhmTuyJWQtAnRX1fVPa7y5v");

  const secretKeynew = JSON.parse(fs.readFileSync("/Users/dm/Documents/fermi_labs/m2/pro/Fermi-Pro/kp3/key.json", "utf-8"));
  const keypairnew = Keypair.fromSecretKey(new Uint8Array(secretKeynew));
  const authority = keypairnew;
  const wallet = new Wallet(keypair);
  const provider = new AnchorProvider(connection, wallet, {});
  const client = new OpenBookV2Client(provider, programId);

  // const payer = authority;
  console.log("keypairnew: ", keypairnew.publicKey.toString())

  

  // wrap authority in an anchor wallet
  //const wallet = new Wallet(keypairnew);


  // Market and Event Heap setup
  const marketPublicKey = new PublicKey("FeBG7uxaGk4GFRPbgqKeeztFk6w5FbqSaoY9BWiKgwou");
  //const eventHeapPublicKey = new PublicKey("BBqeYfvuGFuvD6y9Zcu4Pr5Yon2ajop5cQga8QecTwf2");\
  const market = await client.deserializeMarketAccount(marketPublicKey);
  let eventHeapPublicKey: PublicKey;
  if (market != null) {
  eventHeapPublicKey = new PublicKey(new PublicKey(market.eventHeap.toString()));
  console.log("eventheap is not null {}", eventHeapPublicKey.toString());

  }
  else {  
    eventHeapPublicKey = new PublicKey("BBqeYfvuGFuvD6y9Zcu4Pr5Yon2ajop5cQga8QecTwf2");
    console.log("market is null");
  }
  //const marketAddress = new PublicKey("..."); // replace with actual market address
  const [marketAuthorityPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from('Market'), marketPublicKey.toBuffer()],
    programId,
  );
  console.log("marketAuthorityPDA: ", marketAuthorityPDA.toString());
  // Define the slots to consume (example: [0, 1, 2])
  const slotsToConsume = new BN(0);
  
  // [/* Array of slots to consume */];
  const makerpubkey = keypair.publicKey;
  const takerpubkey = keypairnew.publicKey;

  console.log("makerpubkey: ", makerpubkey.toString());
console.log("takerpubkey: ", takerpubkey.toString());

 
  // Additional accounts setup
  // const makerAtaPublicKey = /* Maker's ATA Public Key */;
  // const takerAtaPublicKey = /* Taker's ATA Public Key */;
  if (market != null) {
  const marketVaultBasePublicKey = market.marketBaseVault;
  /* Market's Base Vault Public Key */
  const marketVaultQuotePublicKey = market.marketQuoteVault;
  const makerAtaPublicKey = new PublicKey(await checkOrCreateAssociatedTokenAccount(provider, market.baseMint, makerpubkey));
  const takerAtaPublicKey = new PublicKey(await checkOrCreateAssociatedTokenAccount(provider, market.baseMint, takerpubkey));
  //const makerOpenOrder = await client.deserializeOpenOrderAccount(makerpubkey);
  const makerOpenOrder = await client.findAllOpenOrders(makerpubkey);
  const makeropenorderfirst = makerOpenOrder[0];
  const takerOpenOrder = await client.findAllOpenOrders(takerpubkey);
    const takeropenorderlast = takerOpenOrder[takerOpenOrder.length - 1];
    console.log("takeropenorderfirst: ", takeropenorderlast.toBase58());
  console.log("makeropenorderfirst: ", makeropenorderfirst.toBase58());
  const makerOO2 = new PublicKey("YxFf7n5bBQYYsWBBxL8EqZ5qM9eDPoETaXjAh5SSCet")

  const accountInfo = await connection.getAccountInfo(eventHeapPublicKey);
  if (accountInfo != null) {
  console.log("eventheap owner is {}", accountInfo.owner.toString());
  }

  /* Market's Quote Vault Public Key */
  // const tokenProgramPublicKey = 
  /* Token Program Public Key */
  //msg!("finalizing events")
  // Create the instruction for finalizing events
  const [ix, signers] = await client.createCancelGivenEventIx(
    Side.Bid, // or { ask: {} } for an ask order
    marketPublicKey,
    marketAuthorityPDA,
    eventHeapPublicKey,
    makerAtaPublicKey,
    takerAtaPublicKey,
    marketVaultBasePublicKey,
    marketVaultQuotePublicKey,
    makeropenorderfirst,
    slotsToConsume,
  );

  // Send transaction
  await client.sendAndConfirmTransaction([ix], {
    //additionalSigners: signers,
  });

  console.log("Events finalized successfully");
}
}
finalizeEvents().catch(console.error);
