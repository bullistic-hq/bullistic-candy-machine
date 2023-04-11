import {
  assertUnreachable,
  Environment,
} from "@bullistic-hq/bullistic-program-shared";
import { Keypair } from "@solana/web3.js";
import { readFileSync } from "fs";

// Note: This can be used to read a keypair from disk in the future if needed.
export default function getWalletKeypairForScript(
  environment: Environment
): Keypair {
  switch (environment) {
    case Environment.Local:
    case Environment.Testnet:
    case Environment.Development:
      return Keypair.generate();
    case Environment.Production: {
      const arr = JSON.parse(
        readFileSync(
          "keys/mainnet/yobh2saMJLYizFSrXscpk4EDUkF1anpZFFzr8JC7KGA.json"
        ).toString()
      );
      return Keypair.fromSecretKey(Uint8Array.from(arr));
    }
    default: {
      return assertUnreachable(environment);
    }
  }
}
