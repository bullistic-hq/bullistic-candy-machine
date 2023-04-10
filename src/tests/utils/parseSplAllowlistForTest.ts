import { PublicKey } from "@solana/web3.js";
import { readFileSync } from "fs";
import SplAllowlistTestConfig from "sdk/types/spl-allowlist-config/SplAllowlistTestConfig";
import SplAllowlistTestConfigJson from "sdk/types/spl-allowlist-config/SplAllowlistTestConfigJson";
import { SPL_TOKEN_ALLOWLIST_MINT_CONFIG } from "tests/constants/allowlistConfig";
import keypairObjectToKeypair from "tests/utils/keypairObjectToKeypair";

export default function parseSplAllowlistForTest(): SplAllowlistTestConfig {
  const filepath = SPL_TOKEN_ALLOWLIST_MINT_CONFIG;
  try {
    const data = readFileSync(filepath, "utf-8");
    const result: SplAllowlistTestConfigJson = JSON.parse(data);
    return {
      allowlistMembers: result.allowlistMembers.map((member) => ({
        address: new PublicKey(member.address),
        balance: member.balance,
      })),
      keypairs: result.keypairs?.map((kp) => keypairObjectToKeypair(kp)),
      splTokenAllowlistMint: new PublicKey(result.splTokenAllowlistMint),
    };
  } catch (err) {
    console.error(err);
    throw new Error(
      `Failed to read SPL allowlist JSON data at path: ${filepath}. Be sure to follow the setup steps in the README first.`
    );
  }
}
