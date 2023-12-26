import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';
import * as spl from '@solana/spl-token';
import { OpenBookV2Client } from '../client'; // Adjust the path as necessary
import { BN, AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { createMint } from './utils2';



const fs = require('fs');


async function initializeMarket() {
  // Initialize connection and client
  //connection to localhost at 8899
  //const {Keypair} = require("@solana/web3.js");
  const secretKey = JSON.parse(fs.readFileSync("/Users/dm/.config/solana/id.json"));
  const keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));
  const authority = keypair;
  const payer = authority;

  // wrap authority in an anchor wallet
  const wallet = new Wallet(keypair);
  //const wallet = anchor.Wallet.local();

  const connection = new Connection("http://localhost:8899", "processed");
  //const connection = new Connection("https://api.devnet.solana.com", "processed");
  //provider setup
  // use default opts.
  const provider = new AnchorProvider(connection, wallet, {});
  //const provider = new OpenBookV2Client(connection);
  //const provider = /* your provider setup */;
  const program_id = new PublicKey("E6cNbXn2BNoMjXUg7biSTYhmTuyJWQtAnRX1fVPa7y5v");
  const client = new OpenBookV2Client(provider, program_id);

  const coinMint = anchor.web3.Keypair.generate();
  const pcMint = anchor.web3.Keypair.generate();

  await createMint(provider, coinMint, 9);
  await createMint(provider, pcMint, 6);
  


  //const payer = Keypair.generate(); // This should be your funded account keypair

  const quoteMint = new PublicKey(coinMint.publicKey.toBase58());
  const baseMint = new PublicKey(pcMint.publicKey.toBase58());

  // Define market parameters
  //quote lot 10e6
  //base lot 10e9
  const quoteLotSize = new BN(1000000);
  const baseLotSize = new BN(1000000000);
  const makerFee = new BN(0);
  const takerFee = new BN(0);
  const timeExpiry = new BN(100000);

  // Create market
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
}

initializeMarket().catch(console.error);
