import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { OpenBookV2Client } from '../client'; // Adjust the path as necessary
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { checkOrCreateAssociatedTokenAccount } from './utils2';

// const fs = require('fs');
import * as fs from 'fs';

// async function finalizeEvents() {
async function finalizeEvents(): Promise<void> {
  // Basic Config
  const secretKey = JSON.parse(
    fs.readFileSync('/Users/dm/.config/solana/id.json', 'utf-8'),
  );
  const keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));
  const wallet = new Wallet(keypair);
  const connection = new Connection('http://localhost:8899', 'processed');
  const provider = new AnchorProvider(connection, wallet, {});
  const programId = new PublicKey(
    'E6cNbXn2BNoMjXUg7biSTYhmTuyJWQtAnRX1fVPa7y5v',
  );
  const client = new OpenBookV2Client(provider, programId);

  // Market and Event Heap setup
  const marketPublicKey = new PublicKey(
    'ATPpcGQEWoh1fGuuY4AkHHGSD3WdHLUXg3XVseQo3K98',
  );
  const eventHeapPublicKey = new PublicKey(
    '8E4MNizP4pkX3Kp97qdKKDuf3Q1Zaa6d7p1Eu8egyNhz',
  );
  const market = await client.deserializeMarketAccount(marketPublicKey);

  // Define the slots to consume (example: [0, 1, 2])
  const slotsToConsume = [0];
  // [/* Array of slots to consume */];
  const makerpubkey = keypair.publicKey;

  // Additional accounts setup
  // const makerAtaPublicKey = /* Maker's ATA Public Key */;
  // const takerAtaPublicKey = /* Taker's ATA Public Key */;
  if (market != null) {
    const marketVaultBasePublicKey = market.marketBaseVault;
    /* Market's Base Vault Public Key */
    const marketVaultQuotePublicKey = market.marketQuoteVault;
    const makerAtaPublicKey = new PublicKey(
      await checkOrCreateAssociatedTokenAccount(
        provider,
        market.baseMint,
        makerpubkey,
      ),
    );
    const takerAtaPublicKey = new PublicKey(
      await checkOrCreateAssociatedTokenAccount(
        provider,
        market.quoteMint,
        makerpubkey,
      ),
    );
    /* Market's Quote Vault Public Key */
    // const tokenProgramPublicKey =
    /* Token Program Public Key */

    // Create the instruction for finalizing events
    const [ix, signers] = await client.createFinalizeEventsInstruction(
      marketPublicKey,
      market,
      eventHeapPublicKey,
      makerAtaPublicKey,
      takerAtaPublicKey,
      marketVaultBasePublicKey,
      marketVaultQuotePublicKey,
      // tokenProgramPublicKey,
      slotsToConsume,
    );

    // Send transaction
    await client.sendAndConfirmTransaction([ix], {
      additionalSigners: signers,
    });

    console.log('Events finalized successfully');
  }
}
finalizeEvents().catch(console.error);
