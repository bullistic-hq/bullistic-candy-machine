import {
  areBuffersEqual,
  deserializeMerkleProof,
  generateKeypairArray,
  randomNumberInRange,
  serializeMerkleProof,
} from "@bullistic-hq/bullistic-program-shared";
import { Keypair } from "@solana/web3.js";
import MerkleAllowlistProof from "sdk/types/MerkleAllowlistProof";
import constructMerkleAllowlist from "sdk/utils/merkle-tree/constructMerkleAllowlist";
import constructMerkleTree from "sdk/utils/merkle-tree/constructMerkleTree";

function assertProofsAreEqual(
  proofA: MerkleAllowlistProof,
  proofB: MerkleAllowlistProof
) {
  proofA.forEach((buffer, i) => {
    expect(areBuffersEqual(buffer, proofB[i])).toBe(true);
  });
}

describe("Merkle tree proof serialization and deserialization", () => {
  test("Test serializeProof and deserializeProof utils", () => {
    const numberOfBuyersToTest = [0, 1, 25];
    numberOfBuyersToTest.forEach((numberOfBuyers) => {
      const keypairs = generateKeypairArray(numberOfBuyers);
      const addresses = keypairs.map((kp) => ({
        address: kp.publicKey,
        amount: randomNumberInRange(0, 5),
      }));
      const candyMachinePubkey = Keypair.generate().publicKey;
      const buyersWithProof = constructMerkleAllowlist(
        candyMachinePubkey,
        addresses
      );

      buyersWithProof.forEach(({ buyersChunk }) => {
        const serializedProofs = buyersChunk.map((buyer) =>
          serializeMerkleProof(buyer.proof)
        );

        const deserializedProofs = serializedProofs.map((proofString) =>
          deserializeMerkleProof(proofString)
        );

        deserializedProofs.forEach((proof, index) => {
          const originalProof = buyersChunk[index].proof;
          assertProofsAreEqual(proof, originalProof);
        });

        const tree = constructMerkleTree(buyersChunk, candyMachinePubkey);
        deserializedProofs.forEach((proof, index) => {
          expect(tree.verifyProof(index, proof, tree.getRoot())).toBe(true);
        });
      });
    });
  });
});
