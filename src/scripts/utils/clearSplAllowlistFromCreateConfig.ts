import { writeFileSync } from "fs";
import { CREATE_CONFIG_FILENAME } from "scripts/constants";
import readCreateConfigFile from "scripts/utils/readCreateConfigFile";

export default function clearSplAllowlistFromCreateConfig() {
  const config = readCreateConfigFile();
  const newConfig = {
    ...config,
    splTokenAllowlistSettings: null,
  };
  writeFileSync(CREATE_CONFIG_FILENAME, JSON.stringify(newConfig, null, 2));
  console.log(
    `Set the create config file ${CREATE_CONFIG_FILENAME} splTokenAllowlistSettings settings to 'null'.`
  );
}
