import { MerkleLeaf } from "@formfunction-hq/formfunction-program-shared";
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";

// Must be consistent with the program MerkleWhitelistProofData.amount size.
const amountNumBytes = 2;

export default function constructMerkleLeafNode(
  buyer: MerkleAllowlistBuyerInfo,
  candyMachinePubkey: PublicKey
): MerkleLeaf {
  return Buffer.from([
    ...buyer.address.toBuffer(),
    ...candyMachinePubkey.toBuffer(),
    ...new BN(buyer.amount).toArray("le", amountNumBytes),
  ]);
}
