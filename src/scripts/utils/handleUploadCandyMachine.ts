import {
  Environment,
  Maybe,
  stringToPublicKey,
} from "@formfunction-hq/formfunction-program-shared";
import { PublicKey } from "@solana/web3.js";
import axios from "axios";
import { API_HEADERS } from "scripts/constants";
import getBackendApiUrlFromEnvironment from "scripts/utils/getBackendApiUrlFromEnvironment";
import readCreateConfigFile from "scripts/utils/readCreateConfigFile";
import MerkleAllowlistConfig from "sdk/types/MerkleAllowlistConfig";
import invariant from "tiny-invariant";

export type CandyMachineAllowlistEntry = {
  address: string;
  amount: number;
  proof: string;
  rootIndex: number;
};

// Note: Keep in sync with API parameters: https://github.com/formfunction-hq/formfn-monorepo/blob/main/packages/server/src/rest/intern/importOnchainCandyMachine.ts#L14
export interface ImportOnchainCandyMachineRequestBody {
  allowlistInfo?: Array<CandyMachineAllowlistEntry>;
  candyMachineAddress: string;
  creatorAuthorityOverride: string;
  logoAssetSrc?: string;
  mintPreviewAssetSrc?: string;
  premintPreviewAssetSrcs?: Array<string>;
  randomizeSeriesSlug: boolean;
}

function convertAllowlistConfig(
  config: Maybe<MerkleAllowlistConfig>
): Array<CandyMachineAllowlistEntry> {
  if (config == null) {
    return [];
  }

  return config.merkleAllowlistData
    .map((section) => {
      return section.buyers.map((buyer) => {
        return {
          address: buyer.address,
          amount: buyer.amount,
          proof: buyer.serializedProof,
          rootIndex: buyer.merkleTreeIndex,
        };
      });
    })
    .flat();
}

export default async function handleUploadCandyMachine(
  candyMachineAddress: PublicKey,
  allowlistConfig: Maybe<MerkleAllowlistConfig>,
  environment: Environment,
  randomizeSeriesSlug: boolean
) {
  const { creatorAuthorityOverride } = readCreateConfigFile();
  invariant(
    stringToPublicKey(creatorAuthorityOverride) != null,
    "A valid PublicKey must be provided as the creatorAuthorityOverride in the config file."
  );

  const apiUrl = getBackendApiUrlFromEnvironment(environment);
  const url = `${apiUrl}/intern/importOnchainCandyMachine`;

  const body: ImportOnchainCandyMachineRequestBody = {
    allowlistInfo: convertAllowlistConfig(allowlistConfig),
    candyMachineAddress: candyMachineAddress.toString(),
    creatorAuthorityOverride,
    logoAssetSrc:
      "https://formfunction.imgix.net/adhoc/popheadz/popheadz-logo.png",
    mintPreviewAssetSrc:
      "https://formfunction.imgix.net/adhoc/popheadz/popheadz-mint-preview.gif",
    premintPreviewAssetSrcs: [
      "https://formfunction.imgix.net/adhoc/popheadz/1.png",
      "https://formfunction.imgix.net/adhoc/popheadz/2.png",
      "https://formfunction.imgix.net/adhoc/popheadz/3.png",
      "https://formfunction.imgix.net/adhoc/popheadz/4.png",
      "https://formfunction.imgix.net/adhoc/popheadz/5.png",
    ],
    randomizeSeriesSlug,
  };

  const response = await axios.post(url, body, { headers: API_HEADERS });
  return response.data;
}
