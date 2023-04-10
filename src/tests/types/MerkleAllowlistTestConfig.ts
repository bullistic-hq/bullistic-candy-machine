import KeypairObject from "tests/types/KeypairObject";
import MerkleAllowlistTestConfigData from "tests/types/MerkleAllowlistTestConfigData";

type MerkleAllowlistTestConfig = {
  candyMachineKeypair: KeypairObject;
  merkleAllowlistData: Array<MerkleAllowlistTestConfigData>;
};

export default MerkleAllowlistTestConfig;
