

import * as anchor from '@project-serum/anchor';
import * as spl from '@solana/spl-token';
import { assert } from 'chai';

const fs = require('fs');

export const createMint = async (
  provider: anchor.AnchorProvider,
  mint: anchor.web3.Keypair,
  decimal: number,
) => {
  //const programId = getDevPgmId();
  const tx = new anchor.web3.Transaction();
  tx.add(
    anchor.web3.SystemProgram.createAccount({
      programId: spl.TOKEN_PROGRAM_ID,
      //programId: programId,
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