import { Keypair } from "@solana/web3.js";
import MerkleAllowlistBuyerWithProof from "sdk/types/MerkleAllowlistBuyerWithProof";

interface AllowlistedBuyerForTest extends MerkleAllowlistBuyerWithProof {
  keypair: Keypair;
}

export default AllowlistedBuyerForTest;
