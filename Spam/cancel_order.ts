import { Keypair, PublicKey } from '@solana/web3.js';
import { OpenBookV2Client } from './path_to_your_client'; // Adjust the path as necessary
import { BN } from '@coral-xyz/anchor';
import { CreateMint } from './utils';

async function cancelOrder() {
  const provider = /* your provider setup */;
  const client = new OpenBookV2Client(provider);

  const marketPublicKey = new PublicKey(/* Market PublicKey */);
  const market = await client.deserializeMarketAccount(marketPublicKey);
  const openOrdersPublicKey = /* Your Open Orders Public Key */;
  const openOrdersAccount = await client.deserializeOpenOrderAccount(openOrdersPublicKey);
  const orderId = new BN(/* order id */);

  

  const [ix, signers] = await client.cancelOrderById(
    openOrdersPublicKey,
    openOrdersAccount,
    market,
    orderId,
  );

  // Send transaction
  await client.sendAndConfirmTransaction([ix], {
    additionalSigners: signers,
  });

  console.log("Order cancelled successfully");
}

cancelOrder().catch(console.error);
