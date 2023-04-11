import { createSplToken } from "@bullistic-hq/bullistic-program-shared";
import { exec } from "child_process";
import { writeFileSync } from "fs";
import parseSplAllowlistConfigArgs from "scripts/utils/env/parseSplAllowlistConfigArgs";
import getCandyMachineSdkForScript from "scripts/utils/getCandyMachineSdkForScript";
import getExplorerLinkForAddress from "scripts/utils/getExplorerNetworkEnvironment";
import getSplAllowlistMembers from "scripts/utils/getSplAllowlistMembers";
import setupAllowlistFolders from "scripts/utils/setupAllowlistFolders";
import writeSplAllowlistSettingsToDeployConfig from "scripts/utils/writeSplAllowlistSettingsToDeployConfig";
import SplAllowlistTestConfigJson from "sdk/types/spl-allowlist-config/SplAllowlistTestConfigJson";
import {
  ALLOWLIST_CONFIG_DIRECTORY,
  SPL_TOKEN_ALLOWLIST_MINT_CONFIG,
} from "tests/constants/allowlistConfig";
import fundSplTokenAtas from "tests/utils/fundSplTokenAtas";
import keypairToObject from "tests/utils/keypairToObject";

async function generateSplAllowlistConfig() {
  const { amount, environment } = parseSplAllowlistConfigArgs();
  setupAllowlistFolders();

  const { connection, wallet } = await getCandyMachineSdkForScript(environment);
  console.log();
  console.log(
    `Generating SPL token allowlist for test using ${environment} environment.`
  );
  console.log(`Paying with wallet ${wallet.publicKey.toString()}.`);

  const splTokenMint = await createSplToken(connection, wallet);
  console.log(`Created SPL token with mint: ${splTokenMint.toString()}.`);
  console.log();

  const { addresses, keypairs } = getSplAllowlistMembers(environment);
  console.log(
    `Sending ${amount} SPL allowlist token(s) to ${addresses.length} allowlist addresses.`
  );
  console.log();
  const allowlistMembers = await fundSplTokenAtas(
    connection,
    addresses,
    splTokenMint,
    wallet,
    amount
  );

  const data: SplAllowlistTestConfigJson = {
    allowlistMembers: allowlistMembers.map((member) => ({
      address: member.wallet.toString(),
      balance: member.balance,
    })),
    keypairs: keypairs?.map((kp) => keypairToObject(kp)),
    splTokenAllowlistMint: splTokenMint.toString(),
  };

  const filepath = SPL_TOKEN_ALLOWLIST_MINT_CONFIG;
  writeFileSync(filepath, JSON.stringify(data, null, 2), "utf-8");
  console.log();
  console.log(`Saved results to: ${filepath}.`);
  console.log();

  exec(`yarn prettier --write "${ALLOWLIST_CONFIG_DIRECTORY}/*.json"`);

  await writeSplAllowlistSettingsToDeployConfig(splTokenMint);

  const explorerLink = getExplorerLinkForAddress(splTokenMint, environment);
  if (explorerLink != null) {
    console.log();
    console.log(`View the token here: ${explorerLink}`);
  }

  console.log();
  console.log("SPL allowlist generation complete.");
  console.log();
}

generateSplAllowlistConfig();
