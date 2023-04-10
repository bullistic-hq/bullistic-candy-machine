import { Keypair } from "@solana/web3.js";
import KeypairObject from "tests/types/KeypairObject";

export default function keypairObjectToKeypair(
  keypairObject: KeypairObject
): Keypair {
  return Keypair.fromSecretKey(new Uint8Array(keypairObject.secretKey));
}
