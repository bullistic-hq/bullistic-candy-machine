import fs from "fs";

const ASSETS_DIR = "./create/assets";

const EXPECTED_METADATA_NAME_PREFIX = "POPHEADZ #";

console.log(`Downshifting all asset in ${ASSETS_DIR} indexes by 1.`);

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

  sorted.forEach(function (file) {
    const filename = `${ASSETS_DIR}/${file}`;
    if (filename.includes("collection")) {
      return;
    }

    const [index, ext] = file.split(".");
    const downshiftedIndex = Number(index) - 1;

    if (ext === "json") {
      const json = fs.readFileSync(filename, "utf-8");
      const data = JSON.parse(json);
      const [_, assetExt] = data.image.split(".");
      const assetFilename = `${downshiftedIndex}.${assetExt}`;
      const revised = {
        ...data,
        image: assetFilename,
        name: `${EXPECTED_METADATA_NAME_PREFIX}${downshiftedIndex}`,
        properties: {
          ...data.properties,
          files: [{ type: `image/${assetExt}`, uri: assetFilename }],
        },
      };
      const newFilename = `${ASSETS_DIR}/${downshiftedIndex}.${ext}`;
      fs.writeFileSync(filename, JSON.stringify(revised, null, 2), "utf-8");
      fs.renameSync(filename, newFilename);
    } else if (ext === "png" || ext === "gif") {
      const downshiftFilename = `${ASSETS_DIR}/${downshiftedIndex}.${ext}`;
      fs.renameSync(filename, downshiftFilename);
    }
  });

  console.log("Done!");
});
