import MerkleAllowlistBuyer from "sdk/types/MerkleAllowlistBuyer";
import KeypairObject from "tests/types/KeypairObject";

interface MerkleAllowlistBuyerForTest extends MerkleAllowlistBuyer {
  keypairObject: KeypairObject;
  proof: Array<Array<number>>;
  serializedProof: string;
}

type MerkleAllowlistTestConfigData = {
  buyers: Array<MerkleAllowlistBuyerForTest>;
  root: Array<number>;
};

export default MerkleAllowlistTestConfigData;
