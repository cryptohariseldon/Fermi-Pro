import * as anchor from '@project-serum/anchor';
import * as spl from '@solana/spl-token';
import { assert } from 'chai';
const { Connection, PublicKey } = require('@solana/web3.js');
// import { Token } from '@solana/spl-token';

const fs = require('fs');

export const createMint = async (
  provider: anchor.AnchorProvider,
  mint: anchor.web3.Keypair,
  decimal: number,
) => {
  // const programId = getDevPgmId();
  const tx = new anchor.web3.Transaction();
  tx.add(
    anchor.web3.SystemProgram.createAccount({
      programId: spl.TOKEN_PROGRAM_ID,
      // programId: programId,
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mint.publicKey,
      space: spl.MintLayout.span,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(
        spl.MintLayout.span,
      ),
    }),
  );
  tx.add(
    spl.createInitializeMintInstruction(
      mint.publicKey,
      decimal,
      provider.wallet.publicKey,
      provider.wallet.publicKey,
    ),
  );
  await provider.sendAndConfirm(tx, [mint]);
};

export const checkOrCreateAssociatedTokenAccount = async (
  provider: anchor.AnchorProvider,
  mint: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey,
) => {
  // Find the ATA for the given mint and owner
  const ata = await spl.getAssociatedTokenAddress(mint, owner, false);

  // Check if the ATA already exists
  const accountInfo = await provider.connection.getAccountInfo(ata);

  if (accountInfo == null) {
    // ATA does not exist, create it
    console.log('Creating Associated Token Account for user...');
    await createAssociatedTokenAccount(provider, mint, ata, owner);
    console.log('Associated Token Account created successfully.');
  } else {
    // ATA already exists
    console.log('Associated Token Account already exists.');
  }

  return ata;
};

export async function checkMintOfATA(connection, ataAddress) {
  try {
    const ataInfo = await connection.getAccountInfo(new PublicKey(ataAddress));
    if (ataInfo === null) {
      throw new Error('Account not found');
    }

    // The mint address is the first 32 bytes of the account data
    const mintAddress = new PublicKey(ataInfo.data.slice(0, 32));
    return mintAddress.toBase58();
  } catch (error) {
    console.error('Error in checkMintOfATA:', error);
    throw error;
  }
}

export const createAssociatedTokenAccount = async (
  provider: anchor.AnchorProvider,
  mint: anchor.web3.PublicKey,
  ata: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey,
) => {
  const tx = new anchor.web3.Transaction();
  tx.add(
    spl.createAssociatedTokenAccountInstruction(
      provider.wallet.publicKey,
      ata,
      owner,
      mint,
    ),
  );
  await provider.sendAndConfirm(tx, []);
};

export const mintTo = async (
  provider: anchor.AnchorProvider,
  mint: anchor.web3.PublicKey,
  ta: anchor.web3.PublicKey,
  amount: bigint,
) => {
  const tx = new anchor.web3.Transaction();
  tx.add(
    spl.createMintToInstruction(
      mint,
      ta,
      provider.wallet.publicKey,
      amount,
      [],
    ),
  );
  await provider.sendAndConfirm(tx, []);
};
