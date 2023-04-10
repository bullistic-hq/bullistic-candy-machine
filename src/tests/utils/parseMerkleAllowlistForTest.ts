import { readFileSync } from "fs";
import { MERKLE_ALLOWLIST_PROGRAM_TEST_CONFIG_FILENAME } from "tests/constants/allowlistConfig";
import MerkleAllowlistTestConfig from "tests/types/MerkleAllowlistTestConfig";

export default function parseMerkleAllowlistForTest(): MerkleAllowlistTestConfig {
  const filepath = MERKLE_ALLOWLIST_PROGRAM_TEST_CONFIG_FILENAME;
  try {
    const data = readFileSync(filepath, "utf-8");
    const result: MerkleAllowlistTestConfig = JSON.parse(data);
    return result;
  } catch (err) {
    console.error(err);
    throw new Error(
      `Failed to read merkle allowlist JSON data at path: ${filepath}. Be sure to follow the setup steps in the README first.`
    );
  }
}
