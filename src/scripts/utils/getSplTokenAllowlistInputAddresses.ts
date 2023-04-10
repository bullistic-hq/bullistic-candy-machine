import { PublicKey } from "@solana/web3.js";
import { readFileSync } from "fs";
import { SPL_TOKEN_ALLOWLIST_INPUT } from "tests/constants/allowlistConfig";

export default function getSplTokenAllowlistInputAddresses(): Array<PublicKey> {
  const filepath = SPL_TOKEN_ALLOWLIST_INPUT;
  try {
    const data = readFileSync(filepath, "utf-8");
    const result: Array<string> = JSON.parse(data);
    return result.map((key) => new PublicKey(key));
  } catch (err) {
    console.error(
      `Received an error trying to read: ${filepath}, error: ${err.message}`
    );
    console.info(
      "Be sure this file exists. It should contain an array of addresses as JSON, for example:"
    );
    console.info(
      JSON.stringify(["4vBpz8WyMuZX6qucrJp8RfhMKivsNiWmq1cp3NRSXFAP"])
    );
    console.log("");
    throw err;
  }
}
