import { Maybe } from "@formfunction-hq/formfunction-program-shared";
import { readFileSync } from "fs";
import MerkleAllowlistConfig from "sdk/types/MerkleAllowlistConfig";
import { MERKLE_ALLOWLIST_CONFIG_FILENAME } from "tests/constants/allowlistConfig";

export default function readMerkleAllowlistConfig(): Maybe<MerkleAllowlistConfig> {
  try {
    const filepath = MERKLE_ALLOWLIST_CONFIG_FILENAME;
    const json = readFileSync(filepath, "utf-8");
    const data: MerkleAllowlistConfig = JSON.parse(json);
    return data;
  } catch (err) {
    console.log("An error occurred reading the merkle allowlist, error: ", err);
    return null;
  }
}
