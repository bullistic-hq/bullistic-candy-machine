import { Maybe } from "@bullistic-hq/bullistic-program-shared";
import fs from "fs";

const ASSETS_DIR = "./create/assets";

const EXPECTED_METADATA_NAME_PREFIX = "POPHEADZ #";
const EXPECTED_CM_SIZE = 21;
const EXPECTED_TOTAL_ASSETS = EXPECTED_CM_SIZE * 2 + 2;

const SUPPORTED_EXTENSIONS = new Set(["json", "png", "gif"]);

let royalties: Maybe<number> = null;
let creators: Maybe<string> = null;

console.log(
  `Validating assets in directory: ${ASSETS_DIR}, expecting ${EXPECTED_CM_SIZE} asset pairs.`
);

fs.readdir(ASSETS_DIR, function (err, files) {
  if (err) {
    console.error(`Could not read the directory: ${ASSETS_DIR}`, err);
    process.exit(1);
  }

  if (
    files.filter((x) => SUPPORTED_EXTENSIONS.has(x.split(".")[1])).length !==
    EXPECTED_TOTAL_ASSETS
  ) {
    throw new Error(
      `Invalid number of assets, expected ${EXPECTED_TOTAL_ASSETS} but got ${files.length}`
    );
  }

  files.forEach(function (file) {
    const filename = `${ASSETS_DIR}/${file}`;
    if (filename.includes("collection")) {
      return;
    }

    const [fileIndex, ext] = file.split(".");

    if (ext === "json") {
      const json = fs.readFileSync(filename, "utf-8");
      const metadata = JSON.parse(json);

      if (royalties == null) {
        royalties = metadata.seller_fee_basis_points;
      }

      if (metadata.seller_fee_basis_points !== royalties) {
        throw new Error(
          `metadata.seller_fee_basis_points is not consistent for all files, first inconsistent file: ${file}`
        );
      }

      const metadataCreators = JSON.stringify(
        metadata.properties.creators.sort()
      );
      if (creators == null) {
        creators = metadataCreators;
      }

      if (metadataCreators !== creators) {
        throw new Error(
          `metadata.properties.creators is not consistent for all files, first inconsistent file: ${file}`
        );
      }

      if (metadata.name !== `${EXPECTED_METADATA_NAME_PREFIX}${fileIndex}`) {
        throw new Error(`metadata name validation failed for file: ${file}`);
      }

      const [_, assetExt] = metadata.image.split(".");
      const expectedImageFileName = `${fileIndex}.${assetExt}`;

      if (metadata.image !== `${fileIndex}.${assetExt}`) {
        throw new Error(`metadata image validation failed for file: ${file}`);
      }

      const assetFileInfo = metadata.properties.files[0];
      if (assetFileInfo == null) {
        throw new Error(`metadata files entries missing for file: ${file}`);
      }

      if (assetFileInfo.uri !== expectedImageFileName) {
        throw new Error(
          `metadata data.properties.files 'uri' field validation failed for file: ${file}`
        );
      }

      if (assetFileInfo.type !== `image/${assetExt}`) {
        throw new Error(
          `metadata data.properties.files 'type' field validation failed for file: ${file}`
        );
      }

      const imageFileExists = fs.existsSync(
        `${ASSETS_DIR}/${expectedImageFileName}`
      );
      if (!imageFileExists) {
        throw new Error(
          `Accompanying image file not found for metadata file: ${file}`
        );
      }
    }
  });

  console.log("Assets validation complete.");
});
