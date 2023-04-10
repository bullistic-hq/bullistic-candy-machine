import { PublicKey } from "@solana/web3.js";
import { readFileSync, writeFileSync } from "fs";
import { CREATE_CONFIG_FILENAME } from "scripts/constants";
import yesno from "yesno";

export default async function writeSplAllowlistSettingsToDeployConfig(
  splTokenMint: PublicKey,
  allowlistMode = "burnEveryTime"
) {
  const ok = await yesno({
    question: `Do you want to update ${CREATE_CONFIG_FILENAME} with the new SPL settings? Type y/n + enter`,
  });
  if (!ok) {
    console.log("Ok, not updating the config.");
    return;
  }

  try {
    const data = readFileSync(CREATE_CONFIG_FILENAME, "utf-8");
    const config = JSON.parse(data);

    const splTokenAllowlistSettings = {
      mint: splTokenMint.toString(),
      mode: allowlistMode,
    };
    const newConfig = {
      ...config,
      splTokenAllowlistSettings,
    };
    writeFileSync(CREATE_CONFIG_FILENAME, JSON.stringify(newConfig, null, 2));
    console.log(`Updated the config file at ${CREATE_CONFIG_FILENAME}`);
  } catch (err) {
    console.error(
      `Failed to read config file at ${CREATE_CONFIG_FILENAME}, this is a no-op now.`
    );
    console.error(err);
  }
}
