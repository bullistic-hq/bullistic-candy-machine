import {
  assertUnreachable,
  Environment,
  Maybe,
} from "@formfunction-hq/formfunction-program-shared";
import { PublicKey } from "@solana/web3.js";

function getLinkWithEnvironmentParam(
  pubkey: PublicKey,
  environmentParam: Maybe<string>
) {
  const params = environmentParam == null ? "" : `?cluster=${environmentParam}`;
  return `https://explorer.solana.com/address/${pubkey.toString()}${params}`;
}

export default function getExplorerLinkForAddress(
  pubkey: PublicKey,
  environment: Environment
): Maybe<string> {
  switch (environment) {
    case Environment.Local:
      return null;
    case Environment.Testnet:
      return getLinkWithEnvironmentParam(pubkey, "testnet");
    case Environment.Development:
      return getLinkWithEnvironmentParam(pubkey, "devnet");
    case Environment.Production: {
      return getLinkWithEnvironmentParam(pubkey, null);
    }
    default: {
      return assertUnreachable(environment);
    }
  }
}
