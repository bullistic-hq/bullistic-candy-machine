type MerkleAllowlistBuyerJson = {
  address: string;
  amount: number;
  merkleTreeIndex: number;
  proof: Array<Array<number>>;
  serializedProof: string;
};

export default MerkleAllowlistBuyerJson;
