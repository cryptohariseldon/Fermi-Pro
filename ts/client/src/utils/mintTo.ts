import * as anchor from "@project-serum/anchor"
import * as spl from "@solana/spl-token"

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