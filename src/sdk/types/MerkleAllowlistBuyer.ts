import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";

type MerkleAllowlistBuyer = MerkleAllowlistBuyerInfo & {
  merkleTreeIndex: number;
};

export default MerkleAllowlistBuyer;
