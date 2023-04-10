import {
  MERKLE_ALLOWLIST_SIZE,
  MERKLE_TREE_LEAF_COUNT_FOR_TESTS,
} from "scripts/constants";
import yargs from "yargs";

type Options = {
  allowlistSize: number;
  leafCount: number;
  useAddressInput: boolean;
};

export default function parseMerkleAllowlistSizeConfigArgs(): Options {
  const { allowlistSize, leafCount, useAddressInput } = yargs(
    process.argv.slice(2)
  )
    .options({
      allowlistSize: {
        default: MERKLE_ALLOWLIST_SIZE,
        type: "number",
      },
      leafCount: { default: MERKLE_TREE_LEAF_COUNT_FOR_TESTS, type: "number" },
      useAddressInput: { default: false, type: "boolean" },
    })
    .parseSync();

  return { allowlistSize, leafCount, useAddressInput };
}
