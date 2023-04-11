import { MerkleTree } from "@bullistic-hq/bullistic-program-shared";
import { PublicKey } from "@solana/web3.js";
import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";
import constructMerkleLeafNode from "sdk/utils/merkle-tree/constructMerkleLeafNode";

export default function constructMerkleTree(
  buyers: Array<MerkleAllowlistBuyerInfo>,
  candyMachinePubkey: PublicKey
): MerkleTree {
  const leafs: Array<Buffer> = [];
  buyers.forEach((buyer) => {
    leafs.push(constructMerkleLeafNode(buyer, candyMachinePubkey));
  });
  return new MerkleTree(leafs);
}
