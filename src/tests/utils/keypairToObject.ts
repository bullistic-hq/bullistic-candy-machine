import { Keypair } from "@solana/web3.js";
import KeypairObject from "tests/types/KeypairObject";

export default function keypairToObject(keypair: Keypair): KeypairObject {
  return {
    publicKey: keypair.publicKey.toString(),
    secretKey: [...keypair.secretKey],
  };
}
