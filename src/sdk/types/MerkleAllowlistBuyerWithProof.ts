import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";
import MerkleAllowlistProof from "sdk/types/MerkleAllowlistProof";

interface MerkleAllowlistBuyerWithProof extends MerkleAllowlistBuyerInfo {
  merkleTreeIndex: number;
  proof: MerkleAllowlistProof;
  serializedProof: string;
}

export default MerkleAllowlistBuyerWithProof;
