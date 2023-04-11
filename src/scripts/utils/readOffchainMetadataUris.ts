import { Maybe } from "@bullistic-hq/bullistic-program-shared";
import { readFileSync } from "fs";
import { METADATA_URIS_FILENAME } from "scripts/constants";

export default function readOffchainMetadataUris(): Maybe<
  Array<{ index: number; uri: string }>
> {
  try {
    const lines = readFileSync(METADATA_URIS_FILENAME, "utf-8");
    return lines
      .split("\n")
      .filter((line) => line !== "")
      .map((line, index) => ({ index, uri: line }));
  } catch (err) {
    console.warn(
      "An error occurred reading the offchain metadata uris, error: ",
      err
    );
    return null;
  }
}
