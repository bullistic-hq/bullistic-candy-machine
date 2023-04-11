import {
  arePublicKeysEqual,
  Maybe,
  stringToPublicKey,
} from "@bullistic-hq/bullistic-program-shared";
import { PublicKey } from "@solana/web3.js";
import parseEnvironmentArg from "scripts/utils/env/parseEnvironmentArg";
import handleUploadCandyMachine from "scripts/utils/handleUploadCandyMachine";
import handleUploadCandyMachineConfigLines from "scripts/utils/handleUploadCandyMachineConfigLines";
import readMerkleAllowlistConfig from "scripts/utils/readMerkleAllowlistData";
import MerkleAllowlistConfig from "sdk/types/MerkleAllowlistConfig";
import parseCandyMachinePubkeyForTest from "tests/utils/parseCandyMachinePubkeyForTest";
import invariant from "tiny-invariant";
import yargs from "yargs";

function parseArgs() {
  const {
    candyMachine,
    includeMerkleAllowlist = false,
    network,
    randomizeSeriesSlug = false,
  } = yargs(process.argv.slice(2))
    .options({
      candyMachine: {
        type: "string",
      },
      includeMerkleAllowlist: {
        type: "boolean",
      },
      network: {
        type: "string",
      },
      randomizeSeriesSlug: {
        type: "boolean",
      },
    })
    .parseSync();

  const candyMachinePublicKey =
    candyMachine != null
      ? stringToPublicKey(candyMachine)
      : parseCandyMachinePubkeyForTest();

  if (candyMachinePublicKey == null) {
    throw new Error(
      "CandyMachine address is required! Provide it like this: '--candyMachine=<address>'"
    );
  }

  const environment = parseEnvironmentArg(network);
  return {
    candyMachine: candyMachinePublicKey,
    environment,
    includeMerkleAllowlist,
    randomizeSeriesSlug,
  };
}

function getAllowlistConfig(
  candyMachine: PublicKey,
  includeMerkleAllowlist: boolean
): Maybe<MerkleAllowlistConfig> {
  if (!includeMerkleAllowlist) {
    return null;
  }
  const allowlist = readMerkleAllowlistConfig();
  invariant(allowlist != null, "Merkle allowlist data must exist.");
  if (
    !arePublicKeysEqual(candyMachine, allowlist.candyMachineKeypair.publicKey)
  ) {
    throw new Error(
      `Provided candyMachine address must match the allowlist candy machine address if importing an allowlist.`
    );
  }

  return allowlist;
}

async function uploadCandyMachine() {
  const {
    candyMachine,
    environment,
    includeMerkleAllowlist,
    randomizeSeriesSlug,
  } = parseArgs();

  console.log(
    `\nUploading config lines for candy machine: ${candyMachine} on network: ${environment}\n`
  );
  await handleUploadCandyMachineConfigLines(candyMachine, environment);

  console.log(
    `\nUploading candy machine data for candy machine: ${candyMachine} on network: ${environment}\n`
  );

  const allowlist = getAllowlistConfig(candyMachine, includeMerkleAllowlist);

  try {
    const result = await handleUploadCandyMachine(
      candyMachine,
      allowlist,
      environment,
      randomizeSeriesSlug
    );
    console.log("Upload complete, result:");
    console.log(result);
  } catch (err) {
    console.log("An error occurred during the uploaded, error:");
    console.log(err);
  }
}

uploadCandyMachine();
