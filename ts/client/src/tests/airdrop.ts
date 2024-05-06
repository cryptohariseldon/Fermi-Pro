import { Helius } from "helius-sdk";
import { PublicKey } from "@solana/web3.js";

const helius = new Helius("f7737f45-0d3f-48a3-983c-8b2338d15df5");
async function initializeMarket(): Promise<void> {

const response = await helius.rpc.airdrop(
  new PublicKey("DCEp8dRr3TeLTcFADbEfHs2iHx6usXE6JhJwzu46M12W"),
  1000000000
); // 1 sol
}

initializeMarket()