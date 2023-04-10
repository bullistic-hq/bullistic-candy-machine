import MerkleAllowlistConfigData from "sdk/types/MerkleAllowlistConfigData";
import KeypairObject from "tests/types/KeypairObject";

type MerkleAllowlistConfig = {
  candyMachineKeypair: KeypairObject;
  merkleAllowlistData: Array<MerkleAllowlistConfigData>;
};

export default MerkleAllowlistConfig;
