import {
  Environment,
  stringToPublicKey,
} from "@formfunction-hq/formfunction-program-shared";
import { PublicKey } from "@solana/web3.js";
import parseEnvironmentArg from "scripts/utils/env/parseEnvironmentArg";
import parseCandyMachinePubkeyForTest from "tests/utils/parseCandyMachinePubkeyForTest";
import yargs from "yargs";

type Options = {
  candyMachine: PublicKey;
  environment: Environment;
};

export default function parsePrintCandyMachineStateArgs(): Options {
  const { candyMachine, environment } = yargs(process.argv.slice(2))
    .options({
      candyMachine: {
        type: "string",
      },
      environment: {
        default: "devnet",
        type: "string",
      },
    })
    .parseSync();

  const candyMachinePublicKey =
    candyMachine != null
      ? stringToPublicKey(candyMachine)
      : parseCandyMachinePubkeyForTest();

  if (candyMachinePublicKey == null) {
    throw new Error("Provided candyMachine address was invalid.");
  }

  return {
    candyMachine: candyMachinePublicKey,
    environment: parseEnvironmentArg(environment),
  };
}
