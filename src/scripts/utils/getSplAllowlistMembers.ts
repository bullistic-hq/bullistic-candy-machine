import {
  assertUnreachable,
  Environment,
  generateKeypairArray,
  Maybe,
} from "@bullistic-hq/bullistic-program-shared";
import { Keypair, PublicKey } from "@solana/web3.js";
import { SPL_ALLOWLIST_SIZE_FOR_TEST } from "scripts/constants";
import getSplTokenAllowlistInputAddresses from "scripts/utils/getSplTokenAllowlistInputAddresses";

// For local testing, we also provide the keypairs so we can test minting
// with the allowlisted address.
type AllowlistMembers = {
  addresses: Array<PublicKey>;
  keypairs: Maybe<Array<Keypair>>;
};

export default function getSplAllowlistMembers(
  environment: Environment
): AllowlistMembers {
  switch (environment) {
    case Environment.Local: {
      const keypairs = generateKeypairArray(SPL_ALLOWLIST_SIZE_FOR_TEST);
      return { addresses: keypairs.map((kp) => kp.publicKey), keypairs };
    }
    case Environment.Development:
    case Environment.Testnet:
    case Environment.Production:
      return {
        addresses: getSplTokenAllowlistInputAddresses(),
        keypairs: null,
      };
    default:
      return assertUnreachable(environment);
  }
}
