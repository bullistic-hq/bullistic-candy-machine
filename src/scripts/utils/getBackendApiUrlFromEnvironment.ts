import {
  assertUnreachable,
  Environment,
} from "@bullistic-hq/bullistic-program-shared";

export default function getBackendApiUrlFromEnvironment(
  environment: Environment
): string {
  switch (environment) {
    case Environment.Local:
      return "http://localhost:4000";
    case Environment.Development:
      return "https://apidev2.bullistic.xyz";
    case Environment.Testnet:
      return "https://apitest.bullistic.xyz";
    case Environment.Production:
      return "https://api2.bullistic.xyz";
    default: {
      return assertUnreachable(environment);
    }
  }
}
