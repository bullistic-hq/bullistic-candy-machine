import { existsSync, mkdirSync } from "fs";
import {
  ALLOWLIST_CONFIG_DIRECTORY,
  ALLOWLIST_CONFIG_INPUT_DIRECTORY,
} from "tests/constants/allowlistConfig";

const foldersToCreate = [
  ALLOWLIST_CONFIG_DIRECTORY,
  ALLOWLIST_CONFIG_INPUT_DIRECTORY,
];

export default function setupAllowlistFolders() {
  foldersToCreate.forEach((dir) => {
    if (!existsSync(dir)) {
      console.log(`${dir} doesn't exist yet, creating it.`);
      mkdirSync(dir);
    }
  });
}
