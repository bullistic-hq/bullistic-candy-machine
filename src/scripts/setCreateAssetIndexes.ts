import fs from "fs";

const ASSETS_DIR = "./create/assets";

const EXPECTED_METADATA_NAME_PREFIX = "TOONIES #";

console.log(`Setting asset indexes in ${ASSETS_DIR} correctly.`);

fs.readdir(ASSETS_DIR, function (err, files) {
  if (err) {
    console.error("Could not list the directory.", err);
    process.exit(1);
  }

  const sorted = files.sort((a, b) => {
    const [indexA] = a.replace(/ASSETS_DIR/, "").split(".");
    const [indexB] = b.replace(/ASSETS_DIR/, "").split(".");
    return Number(indexA) - Number(indexB);
  });

  let currentAssetIndex = 0;

  sorted.forEach(function (file) {
    const filename = `${ASSETS_DIR}/${file}`;
    if (filename.includes("collection")) {
      return;
    }

    const [_, ext] = file.split(".");

    if (ext === "json") {
      const json = fs.readFileSync(filename, "utf-8");
      const data = JSON.parse(json);
      const [__, assetExt] = data.image.split(".");
      const assetFilename = `${currentAssetIndex}.${assetExt}`;
      const revised = {
        ...data,
        image: assetFilename,
        name: `${EXPECTED_METADATA_NAME_PREFIX}${currentAssetIndex}`,
        properties: {
          ...data.properties,
          files: [{ type: `image/${assetExt}`, uri: assetFilename }],
        },
      };
      const newFilename = `${ASSETS_DIR}/${currentAssetIndex}.${ext}`;
      fs.writeFileSync(filename, JSON.stringify(revised, null, 2), "utf-8");
      fs.renameSync(filename, newFilename);
      console.log(`Saved ${newFilename}`);
      currentAssetIndex++;
    }
  });

  console.log("Done!");
});
