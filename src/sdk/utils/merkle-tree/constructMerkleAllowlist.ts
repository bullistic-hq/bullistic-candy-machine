import {
  chunkArray,
  MerkleTree,
  serializeMerkleProof,
} from "@formfunction-hq/formfunction-program-shared";
import { PublicKey } from "@solana/web3.js";
import MERKLE_TREE_LEAF_COUNT_LIMIT from "sdk/constants/MerkleTreeLeafCountLimit";
import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";
import MerkleAllowlistBuyerWithProof from "sdk/types/MerkleAllowlistBuyerWithProof";
import constructMerkleTree from "sdk/utils/merkle-tree/constructMerkleTree";

type MerkleAllowlistBuyersListForTest = {
  buyersChunk: Array<MerkleAllowlistBuyerWithProof>;
  tree: MerkleTree;
};

export default function constructMerkleAllowlist(
  candyMachinePubkey: PublicKey,
  buyers: Array<MerkleAllowlistBuyerInfo>,
  merkleTreeLeafSizeLimit = MERKLE_TREE_LEAF_COUNT_LIMIT
): Array<MerkleAllowlistBuyersListForTest> {
  return chunkArray(buyers, merkleTreeLeafSizeLimit).map(
    (chunk, merkleTreeIndex) => {
      const tree = constructMerkleTree(chunk, candyMachinePubkey);
      const buyersChunk = chunk.map((buyer, index) => {
        const proof = tree.getProof(index);
        const { amount, address } = buyer;
        return {
          address,
          amount,
          merkleTreeIndex,
          proof,
          serializedProof: serializeMerkleProof(proof),
        };
      });
      return { buyersChunk, tree };
    }
  );
}
