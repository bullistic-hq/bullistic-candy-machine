import {
  chunkArray,
  MerkleTree,
} from "@bullistic-hq/bullistic-program-shared";
import { PublicKey } from "@solana/web3.js";
import MERKLE_TREE_LEAF_COUNT_LIMIT from "sdk/constants/MerkleTreeLeafCountLimit";
import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";
import constructMerkleTree from "sdk/utils/merkle-tree/constructMerkleTree";

export default function constructMerkleTrees(
  buyers: Array<MerkleAllowlistBuyerInfo>,
  candyMachinePubkey: PublicKey,
  treeLeafCountLimit = MERKLE_TREE_LEAF_COUNT_LIMIT
): Array<MerkleTree> {
  return chunkArray(buyers, treeLeafCountLimit).map((buyersChunk) =>
    constructMerkleTree(buyersChunk, candyMachinePubkey)
  );
}
