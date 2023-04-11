import { Environment } from "@bullistic-hq/bullistic-program-shared";
import parseEnvironmentArg from "scripts/utils/env/parseEnvironmentArg";
import yargs from "yargs";

type Options = {
  amount: number;
  environment: Environment;
};

export default function parseSplAllowlistConfigArgs(): Options {
  const { amount, environment } = yargs(process.argv.slice(2))
    .options({
      amount: {
        default: 10,
        type: "number",
      },
      environment: {
        default: Environment.Local,
        type: "string",
      },
    })
    .parseSync();

  return { amount, environment: parseEnvironmentArg(environment) };
}
