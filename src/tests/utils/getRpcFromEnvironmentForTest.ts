import {
  assertUnreachable,
  Environment,
} from "@bullistic-hq/bullistic-program-shared";

export default function getRpcFromEnvironmentForTest(
  environment: Environment
): string {
  switch (environment) {
    case Environment.Testnet:
      return "https://api.testnet.solana.com";
    case Environment.Development:
      // Backup, lol.
      // return "https://solana-devnet.g.alchemy.com/v2/8RpkBkJ5LtlzfguQ18eUvVImfc7_8p_y";
      return "https://api.devnet.solana.com";
    case Environment.Local:
      return "http://localhost:8899";
    case Environment.Production:
      return "https://patient-proud-dew.solana-mainnet.quiknode.pro/5d277ae9935d3f2c1513be336f0d0bcb54f63a07/";
    default: {
      return assertUnreachable(environment);
    }
  }
}
