import {
  generateKeypairArray,
  Maybe,
  randomNumberInRange,
} from "@bullistic-hq/bullistic-program-shared";
import { Keypair } from "@solana/web3.js";
import getMerkleAllowlistInputAddresses from "scripts/utils/getMerkleAllowlistInputAddresses";
import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";

export default function getAddressListForMerkleAllowlistGeneration(
  useAddressInput: boolean,
  allowlistSize: number
): {
  addresses: Array<MerkleAllowlistBuyerInfo>;
  keypairs: Maybe<Array<Keypair>>;
} {
  if (useAddressInput === false) {
    const keypairs = generateKeypairArray(allowlistSize);
    const addresses = keypairs.map((kp) => ({
      address: kp.publicKey,
      amount: randomNumberInRange(0, 5),
    }));
    return { addresses, keypairs };
  }

  const addresses = getMerkleAllowlistInputAddresses();
  return { addresses, keypairs: null };
}
