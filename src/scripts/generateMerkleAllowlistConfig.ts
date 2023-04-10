import { treeNodeToArray } from "@formfunction-hq/formfunction-program-shared";
import { Keypair, PublicKey } from "@solana/web3.js";
import { exec } from "child_process";
import { writeFileSync } from "fs";
import clearSplAllowlistFromCreateConfig from "scripts/utils/clearSplAllowlistFromCreateConfig";
import parseMerkleAllowlistSizeConfigArgs from "scripts/utils/env/parseMerkleAllowlistSizeConfigArgs";
import getAddressListForMerkleAllowlistGeneration from "scripts/utils/getAddressListForMerkleAllowlistGeneration";
import setupAllowlistFolders from "scripts/utils/setupAllowlistFolders";
import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";
import MerkleAllowlistConfig from "sdk/types/MerkleAllowlistConfig";
import MerkleAllowlistConfigData from "sdk/types/MerkleAllowlistConfigData";
import constructMerkleAllowlist from "sdk/utils/merkle-tree/constructMerkleAllowlist";
import {
  ALLOWLIST_CONFIG_DIRECTORY,
  CANDY_MACHINE_PUBKEY,
  MERKLE_ALLOWLIST_CONFIG_FILENAME,
  MERKLE_ALLOWLIST_INPUT,
  MERKLE_ALLOWLIST_PROGRAM_TEST_CONFIG_FILENAME,
} from "tests/constants/allowlistConfig";
import MerkleAllowlistTestConfig from "tests/types/MerkleAllowlistTestConfig";
import MerkleAllowlistTestConfigData from "tests/types/MerkleAllowlistTestConfigData";
import keypairToObject from "tests/utils/keypairToObject";
import invariant from "tiny-invariant";

function handleGenerateMerkleAllowlistProgramTestConfig(
  candyMachineKeypair: Keypair,
  buyerKeypairs: Array<Keypair>,
  allowlistBuyers: Array<MerkleAllowlistBuyerInfo>,
  leafCount: number
) {
  const buyersWithProof = constructMerkleAllowlist(
    candyMachineKeypair.publicKey,
    allowlistBuyers,
    leafCount
  );

  let allowlistBuyerIndex = 0;
  const allowlistDataForTest: Array<MerkleAllowlistTestConfigData> =
    buyersWithProof.map(({ tree, buyersChunk }) => {
      const root = tree.getRoot();
      const buyers = buyersChunk.map((buyer, index) => {
        invariant(
          tree.verifyProof(index, buyer.proof, root),
          "Generated merkle allowlist proof must be valid."
        );
        const result = {
          ...buyer,
          keypairObject: keypairToObject(buyerKeypairs[allowlistBuyerIndex]),
          proof: buyer.proof.map((val) => treeNodeToArray(val)),
        };

        allowlistBuyerIndex++;
        return result;
      });
      return {
        buyers,
        root: treeNodeToArray(root),
      };
    });

  const data: MerkleAllowlistTestConfig = {
    candyMachineKeypair: keypairToObject(candyMachineKeypair),
    merkleAllowlistData: allowlistDataForTest,
  };

  const filepath = MERKLE_ALLOWLIST_PROGRAM_TEST_CONFIG_FILENAME;
  writeFileSync(filepath, JSON.stringify(data, null, 2), "utf-8");
  console.log(`Saved merkle allowlist program test config to ${filepath}`);
}

function saveCandyMachinePubkey(candyMachinePubkey: PublicKey) {
  const filepath = CANDY_MACHINE_PUBKEY;
  const data = {
    candyMachinePubkey: candyMachinePubkey.toString(),
  };
  writeFileSync(filepath, JSON.stringify(data, null, 2), "utf-8");
  console.log(`Saved merkle allowlist program test config to ${filepath}`);
}

function handleGenerateMerkleAllowlistCliConfig(
  candyMachineKeypair: Keypair,
  allowlistBuyers: Array<MerkleAllowlistBuyerInfo>,
  leafCount: number
) {
  const buyersWithProof = constructMerkleAllowlist(
    candyMachineKeypair.publicKey,
    allowlistBuyers,
    leafCount
  );

  const merkleAllowlistData: Array<MerkleAllowlistConfigData> =
    buyersWithProof.map(({ tree, buyersChunk }) => {
      const root = tree.getRoot();
      const buyers = buyersChunk.map((buyer, index) => {
        invariant(
          tree.verifyProof(index, buyer.proof, root),
          "Generated merkle allowlist proof must be valid."
        );
        return {
          ...buyer,
          address: buyer.address.toString(),
          proof: buyer.proof.map((val) => treeNodeToArray(val)),
        };
      });
      return {
        buyers,
        root: treeNodeToArray(root),
      };
    });

  const data: MerkleAllowlistConfig = {
    candyMachineKeypair: keypairToObject(candyMachineKeypair),
    merkleAllowlistData,
  };

  const filepath = MERKLE_ALLOWLIST_CONFIG_FILENAME;
  writeFileSync(filepath, JSON.stringify(data, null, 2), "utf-8");
  console.log(`Saved merkle allowlist CLI config to ${filepath}`);
}

function generateMerkleAllowlistConfig() {
  const { allowlistSize, leafCount, useAddressInput } =
    parseMerkleAllowlistSizeConfigArgs();

  console.log();
  console.log("Merkle allowlist generation config:");
  console.log();
  if (!useAddressInput) {
    console.log(
      "Running generation for tests only - allowlisted addresses will be generated randomly."
    );
    console.log(`Allowlist size = ${allowlistSize}`);
    console.log(`Tree leaf size limit = ${leafCount}`);
    console.log(
      `Number of tree roots = ${Math.ceil(allowlistSize / leafCount)}`
    );
  } else {
    console.log(
      `Using custom address input from file: ${MERKLE_ALLOWLIST_INPUT}`
    );
  }
  console.log();

  setupAllowlistFolders();

  const candyMachineKeypair = Keypair.generate();
  const { addresses, keypairs } = getAddressListForMerkleAllowlistGeneration(
    useAddressInput,
    allowlistSize
  );

  console.log(`Generating merkle allowlist for ${addresses.length} addresses.`);
  console.log();

  if (!useAddressInput) {
    invariant(
      keypairs != null,
      "Keypairs should exist if generating a merkle allowlist for program tests."
    );
    handleGenerateMerkleAllowlistProgramTestConfig(
      candyMachineKeypair,
      keypairs,
      addresses,
      leafCount
    );
  }

  handleGenerateMerkleAllowlistCliConfig(
    candyMachineKeypair,
    addresses,
    leafCount
  );

  // Remove the SPL allowlist settings from the create config file if the
  // merkle generation is using the custom address input.
  if (useAddressInput) {
    clearSplAllowlistFromCreateConfig();
  }

  saveCandyMachinePubkey(candyMachineKeypair.publicKey);

  exec(`yarn prettier --write "${ALLOWLIST_CONFIG_DIRECTORY}/*.json"`);

  console.log("\nMerkle allowlist generation complete.\n");
}

generateMerkleAllowlistConfig();
