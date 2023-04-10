import {
  assertUnreachable,
  Environment,
} from "@formfunction-hq/formfunction-program-shared";

export default function getBackendApiUrlFromEnvironment(
  environment: Environment
): string {
  switch (environment) {
    case Environment.Local:
      return "http://localhost:4000";
    case Environment.Development:
      return "https://apidev2.formfunction.xyz";
    case Environment.Testnet:
      return "https://apitest.formfunction.xyz";
    case Environment.Production:
      return "https://api2.formfunction.xyz";
    default: {
      return assertUnreachable(environment);
    }
  }
}
