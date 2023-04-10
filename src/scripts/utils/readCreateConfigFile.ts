import { readFileSync } from "fs";
import { CREATE_CONFIG_FILENAME } from "scripts/constants";

export default function readCreateConfigFile() {
  const filepath = CREATE_CONFIG_FILENAME;
  try {
    const data = readFileSync(filepath, "utf-8");
    return JSON.parse(data);
  } catch (err) {
    console.error(
      `Error reading create config file at ${filepath}. Please check the file exists.`
    );
  }
}
