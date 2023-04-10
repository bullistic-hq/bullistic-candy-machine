import {
  chunkArray,
  generateKeypairArray,
  randomNumberInRange,
} from "@formfunction-hq/formfunction-program-shared";
import { Keypair } from "@solana/web3.js";
import constructMerkleAllowlist from "sdk/utils/merkle-tree/constructMerkleAllowlist";

describe("Multi-merkle tree and allowlist construction", () => {
  test("Tree construction and proofs are correct for various allowlist sizes", () => {
    const allowlistSizes = [0, 1, 2, 3, 12, 53, 118, 259, 5_013];

    allowlistSizes.forEach((size) => {
      const candyMachinePubkey = Keypair.generate();
      const allowlistBuyerKeypairs = generateKeypairArray(size);
      const treeLeafLimit = 5;
      const addresses = allowlistBuyerKeypairs.map((kp) => ({
        address: kp.publicKey,
        amount: randomNumberInRange(0, 5),
      }));

      const allowlistBuyersList = constructMerkleAllowlist(
        candyMachinePubkey.publicKey,
        addresses,
        treeLeafLimit
      );

      const expectedNumberOfProofs = Math.ceil(size / treeLeafLimit);
      expect(allowlistBuyersList.length).toBe(expectedNumberOfProofs);
      expect(
        allowlistBuyersList.map((val) => val.buyersChunk).flat().length
      ).toBe(size);

      chunkArray(allowlistBuyersList, treeLeafLimit).forEach((buyersList) => {
        buyersList.forEach(({ tree, buyersChunk }) => {
          const root = tree.getRoot();

          const shiftedBuyersChunk = [...buyersChunk.slice(1), buyersChunk[0]];
          if (shiftedBuyersChunk.length > 1) {
            shiftedBuyersChunk.forEach(({ proof }, buyerIndex) => {
              expect(tree.verifyProof(buyerIndex, proof, root)).toBe(false);
            });
          }

          buyersChunk.forEach(({ proof }, buyerIndex) => {
            expect(tree.verifyProof(buyerIndex, proof, root)).toBe(true);
          });
        });
      });
    });
  });
});
