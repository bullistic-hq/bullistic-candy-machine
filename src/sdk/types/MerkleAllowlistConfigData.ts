import MerkleAllowlistBuyerJson from "sdk/types/merkle-allowlist-config/MerkleAllowlistBuyerJson";

type MerkleAllowlistConfigData = {
  buyers: Array<MerkleAllowlistBuyerJson>;
  root: Array<number>;
};

export default MerkleAllowlistConfigData;
