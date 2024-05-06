import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { EventHeapAccount, FermiClient, FillEvent, OutEvent, getLocalKeypair } from "../src";
import { Connection, PublicKey } from "@solana/web3.js";
//import { programId, rpcUrl } from "./constants";

export const initClientWithKeypairPath = (path: string) => {
  const authority = getLocalKeypair(path);

  // wrap authority keypair in an anchor wallet
  const wallet = new Wallet(authority);

  const conn = new Connection(rpcUrl);
  const provider = new AnchorProvider(conn, wallet, {
    commitment: "finalized",
  });
  const client = new FermiClient(provider, new PublicKey(programId),{
    postSendTxCallback:(tx)=>console.log("Tx Sent:",`https://solana.fm/tx/${tx.txid}?cluster=devnet-solana`)
  });

  return client
};


export const parseEventHeap = (
  client: FermiClient,
  eventHeap: EventHeapAccount | null,
) => {
  if (eventHeap == null) throw new Error("Event Heap not found");
  const  fillEvents: any = [];
  const outEvents: any = [];
  // let nodes: any = [];
  if (eventHeap !== null) {
    (eventHeap.nodes as any).forEach((node: any, i: number) => {
      // nodes.push(node.event);
      if (node.event.eventType === 0) {
        const fillEvent: FillEvent = client.program.coder.types.decode(
          "FillEvent",
          Buffer.from([0, ...node.event.padding]),
        );
        if (fillEvent.timestamp.toString() !== "0") {
          fillEvents.push({
            ...fillEvent,
            index: i,
            maker: fillEvent.maker.toString(),
            taker: fillEvent.taker.toString(),
            price: fillEvent.price.toString(),
            quantity: fillEvent.quantity.toString(),
            makerClientOrderId: fillEvent.makerClientOrderId.toString(),
            takerClientOrderId: fillEvent.takerClientOrderId.toString(),
          });
        }
      } else {
        const outEvent: OutEvent = client.program.coder.types.decode(
          "OutEvent",
          Buffer.from([0, ...node.event.padding]),
        );
        
        if (outEvent.timestamp.toString() !== "0")
          outEvents.push({ ...outEvent, index: i, });
      }
    });
  }

  return {fillEvents,outEvents};
};
