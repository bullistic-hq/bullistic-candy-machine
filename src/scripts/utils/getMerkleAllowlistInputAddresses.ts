import { stringToPublicKey } from "@bullistic-hq/bullistic-program-shared";
import { readFileSync } from "fs";
import { DEFAULT_MERKLE_ALLOWLIST_AMOUNT } from "scripts/constants";
import isPositiveInteger from "scripts/utils/isPositiveInteger";
import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";
import { MERKLE_ALLOWLIST_INPUT } from "tests/constants/allowlistConfig";
import invariant from "tiny-invariant";

type MerkleAllowlistBuyerInfoJsonInput = {
  address: string;
  amount?: number;
};

function parseAndValidateJsonAddressInput(
  data: Array<MerkleAllowlistBuyerInfoJsonInput>
): Array<MerkleAllowlistBuyerInfo> {
  const addressSet = new Set();

  return data.map((info) => {
    const amount =
      info.amount != null
        ? Number(info.amount)
        : DEFAULT_MERKLE_ALLOWLIST_AMOUNT;
    invariant(
      isPositiveInteger(amount),
      `Allowlisted amount must be a positive integer. Received ${info.amount} for address ${info.address}.`
    );

    const address = stringToPublicKey(info.address);
    invariant(
      address != null,
      `Address must be a valid PublicKey, ${info.address} was not.`
    );

    invariant(
      !addressSet.has(address.toString()),
      `Addresses do not need to be listed multiple times in the input list. Found ${address} more than once.`
    );
    addressSet.add(address.toString());

    return {
      address,
      amount,
    };
  });
}

export default function getMerkleAllowlistInputAddresses(): Array<MerkleAllowlistBuyerInfo> {
  const filepath = MERKLE_ALLOWLIST_INPUT;
  try {
    const json = readFileSync(filepath, "utf-8");
    const data: Array<MerkleAllowlistBuyerInfoJsonInput> = JSON.parse(json);
    return parseAndValidateJsonAddressInput(data);
  } catch (err) {
    console.error(
      `Received an error trying to read: ${filepath}, error: ${err.message}`
    );
    console.info(
      "Be sure this file exists. It should contain an array of addresses and amounts as JSON, for example:"
    );
    console.info(
      JSON.stringify([
        {
          address: "CS11P12u5dkyi7L4S41SRa44vgTpFMdQAgHvi1LFrXUu",
          amount: 4,
        },
      ])
    );
    console.info(
      `Amount is optional. If omitted amount will be default to ${DEFAULT_MERKLE_ALLOWLIST_AMOUNT}.`
    );
    console.log();
    throw err;
  }
}
