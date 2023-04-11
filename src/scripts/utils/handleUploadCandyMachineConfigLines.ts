import {
  chunkArray,
  Environment,
} from "@bullistic-hq/bullistic-program-shared";
import { PublicKey } from "@solana/web3.js";
import axios from "axios";
import pLimit from "p-limit";
import { API_HEADERS } from "scripts/constants";
import getBackendApiUrlFromEnvironment from "scripts/utils/getBackendApiUrlFromEnvironment";
import readOffchainMetadataUris from "scripts/utils/readOffchainMetadataUris";
import sleep from "scripts/utils/sleep";

const limitUploadConcurrency = pLimit(10);

const BATCH_SIZE = 100;

const RETRY_DELAY_MS = 1000;

type UriBatch = Array<{ index: number; uri: string }>;

// Note: Keep in sync with API parameters: https://github.com/bullistic-hq/bullistic-monorepo/blob/main/packages/server/src/rest/intern/writeCandyMachineInfoToFirestoreEndpoint.ts
type RequestBody = {
  candyMachineAddress: string;
  offchainMetadataUris: UriBatch;
};

async function handleBatchUpload(args: {
  batch: UriBatch;
  batchIndex: number;
  candyMachineAddress: PublicKey;
  uploadUrl: string;
}): Promise<void> {
  const { uploadUrl, candyMachineAddress, batch, batchIndex } = args;
  const start = batch[0].index;
  const end = batch[batch.length - 1].index;
  try {
    console.log(
      `Uploading ${batch.length} config lines from batch ${batchIndex} (lines in range [${start}, ${end}])...`
    );
    const body: RequestBody = {
      candyMachineAddress: candyMachineAddress.toString(),
      offchainMetadataUris: batch,
    };
    await axios.post(uploadUrl, body, { headers: API_HEADERS });
    console.log(`Batch ${batchIndex} uploaded successfully!`);
  } catch (e) {
    console.error(`Error, HTTP status code = ${e.response.status}`);
    console.error(
      `An error occurred while uploading batch ${batchIndex} (lines in range [${start}, ${end}]), retrying in ${RETRY_DELAY_MS}ms...`
    );
    await sleep(RETRY_DELAY_MS);
    await handleBatchUpload(args);
  }
}

export default async function handleUploadCandyMachineConfigLines(
  candyMachineAddress: PublicKey,
  environment: Environment
) {
  const offchainMetadataUris = readOffchainMetadataUris();
  if (offchainMetadataUris == null) {
    return null;
  }

  const apiUrl = getBackendApiUrlFromEnvironment(environment);
  const uploadUrl = `${apiUrl}/intern/writeCandyMachineInfoToFirestore`;

  const batches = chunkArray(offchainMetadataUris, BATCH_SIZE);
  console.log(
    `Starting upload of ${batches.length} config line batches for ${offchainMetadataUris.length} total metadata uris.\n`
  );
  await Promise.all(
    batches.map((batch, batchIndex) =>
      limitUploadConcurrency(async () => {
        await handleBatchUpload({
          batch,
          batchIndex,
          candyMachineAddress,
          uploadUrl,
        });
      })
    )
  );

  try {
    await axios.post(
      `${apiUrl}/intern/writeCandyMachineInfoWithRarityToFirestore`,
      {
        candyMachineAddress: candyMachineAddress.toString(),
      },
      {
        headers: API_HEADERS,
      }
    );
  } catch (err) {
    console.error("Error updating rarity info.");
    console.error(err);
    throw err;
  }
}
