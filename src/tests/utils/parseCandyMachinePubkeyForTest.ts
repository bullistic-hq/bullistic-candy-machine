import { PublicKey } from "@solana/web3.js";
import { readFileSync } from "fs";
import { CANDY_MACHINE_PUBKEY } from "tests/constants/allowlistConfig";

type CandyMachinePubkeyJson = {
  candyMachinePubkey: string;
};

export default function parseCandyMachinePubkeyForTest(): PublicKey {
  const filepath = CANDY_MACHINE_PUBKEY;
  try {
    const data = readFileSync(filepath, "utf-8");
    const result: CandyMachinePubkeyJson = JSON.parse(data);
    return new PublicKey(result.candyMachinePubkey);
  } catch (err) {
    console.error(err);
    throw new Error(
      `Failed to read candy machine pubkey JSON file at path: ${filepath}. Be sure to follow the setup steps in the README first.`
    );
  }
}
