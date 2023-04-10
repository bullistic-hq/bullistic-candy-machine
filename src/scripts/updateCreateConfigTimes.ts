import dayjs from "dayjs";
import { writeFileSync } from "fs";
import { CREATE_CONFIG_FILENAME } from "scripts/constants";
import readCreateConfigFile from "scripts/utils/readCreateConfigFile";
import yargs from "yargs";

///
/// This script makes it easy to modify the config with
/// allowlistSaleStartTime and publicSaleStartTime values that are a few minutes
/// into the future (so that all 3 phases can be manually tested).
///

const config = readCreateConfigFile();

const { allowlistMinutesAhead = 5, publicSaleMinutesAhead = 10 } = yargs(
  process.argv.slice(2)
)
  .options({
    allowlistMinutesAhead: {
      type: "number",
    },
    publicSaleMinutesAhead: {
      type: "number",
    },
  })
  .parseSync();

const newConfig = {
  ...config,
  allowlistSaleStartTime: dayjs()
    // Add 1 to account for rounding down with startOf
    .add(allowlistMinutesAhead + 1, "minutes")
    .startOf("minute")
    .format(),
  publicSaleStartTime: dayjs()
    // Add 1 to account for rounding down with startOf
    .add(publicSaleMinutesAhead + 1, "minutes")
    .startOf("minute")
    .format(),
};
writeFileSync(CREATE_CONFIG_FILENAME, JSON.stringify(newConfig, null, 2));
console.log(`Updated the config file at ${CREATE_CONFIG_FILENAME}`);
