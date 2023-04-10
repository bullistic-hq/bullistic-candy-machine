import MerkleAllowlistAddressAndAmount from "sdk/types/merkle-allowlist-config/MerkleAllowlistAddressAndAmount";
import KeypairObject from "tests/types/KeypairObject";

type MerkleAllowlistAddressListConfig = {
  candyMachineKeypair: KeypairObject;
  merkleAllowlistAddresses: Array<MerkleAllowlistAddressAndAmount>;
};

export default MerkleAllowlistAddressListConfig;
