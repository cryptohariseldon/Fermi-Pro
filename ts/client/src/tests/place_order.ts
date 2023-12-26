import { Keypair, PublicKey } from '@solana/web3.js';
import { OpenBookV2Client, Side } from './path_to_your_client'; // Adjust the path as necessary
import { BN } from '@coral-xyz/anchor';

// Constants:
/*
Market account: HQs67xA79oj21nYnyADw1VJLMDwNAQP5df8djoVQjNf4
Bids account: AjLRT9cXHznHrp6L5xF8giDHwyKdoCdY2BkCSahQ8Brr
Asks account: 3iXrpp65Vfg2euZMPAqTahxkT2kKziqzzHx9RjWV6fzH
Event heap account: AD2sorSwN14grpGjZzL2Pu4K3JfDKuwLiAPNLoHDfRzs
Quote mint: AaqTC7PhHWm1m4EkkD87Tw1Sp4GN2kzx1HfEThryPAA8
Base mint: HxsoDEUddBjhZB4CLSFewFph72qDMksoKNxV2WVHbWgt
*/

async function placeOrder() {
  const provider = /* your provider setup */;
  const client = new OpenBookV2Client(provider);



  const marketPublicKey = new PublicKey(/* Market PublicKey */);
  const market = await client.deserializeMarketAccount(marketPublicKey);
  const openOrdersPublicKey = /* Your Open Orders Public Key */;
  const userTokenAccount = /* Your Token Account Public Key */;
  const orderArgs = {
    side: Side.Bid, // or Side.Ask
    limitPrice: new BN(/* price */),
    maxBaseQuantity: new BN(/* quantity */),
    maxQuoteQuantity: new BN(/* quantity */),
    clientOrderId: new BN(/* client order id */),
    selfTradeBehavior: /* self trade behavior */,
    orderType: /* order type */,
    limit: /* limit */,
  };

  const [ix, signers] = await client.placeOrderIx(
    openOrdersPublicKey,
    marketPublicKey,
    market,
    userTokenAccount,
    null, // openOrdersAdmin
    orderArgs,
    [], // remainingAccounts
  );

  // Send transaction
  await client.sendAndConfirmTransaction([ix], {
    additionalSigners: signers,
  });

  console.log("Order placed successfully");
}

placeOrder().catch(console.error);
