import { generateKeypairArray } from "@bullistic-hq/bullistic-program-shared";
import { Keypair } from "@solana/web3.js";
import MerkleAllowlistBuyerInfo from "sdk/types/MerkleAllowlistBuyerInfo";
import constructMerkleAllowlist from "sdk/utils/merkle-tree/constructMerkleAllowlist";

const BYTES_PER_PROOF_ELEM = 32;

const MAX_ALLOWLIST_SIZE_TO_ESTIMATE = 10_000;

// Using the formula makes it easy to calculate larger sizes.
const MAX_ALLOWLIST_SIZE_TO_ESTIMATE_WITH_FORMULA = 1_000_000;

type TreeLeafCount = number;
type ProofLength = number;
type ProofSizeInBytes = number;

type ProofSizeResult = {
  ["Leaf count per tree"]: TreeLeafCount;
  ["Proof length"]: ProofLength;
  ["Proof size in bytes"]: ProofSizeInBytes;
};

// Using formula: *************************************************************

function estimateProofSizesWithFormula(
  allowlistSize: TreeLeafCount,
  result: Array<ProofSizeResult>,
  lastProofSize: ProofLength
): Array<ProofSizeResult> {
  if (allowlistSize > MAX_ALLOWLIST_SIZE_TO_ESTIMATE_WITH_FORMULA) {
    return result;
  }

  const proofSize = lastProofSize + 1;
  result.push({
    ["Leaf count per tree"]: allowlistSize,
    ["Proof length"]: proofSize,
    ["Proof size in bytes"]: proofSize * BYTES_PER_PROOF_ELEM,
  });

  // See details here for calculation: https://github.com/bullistic-hq/bullistic-candy-machine/pull/61
  const nextAllowlistSize =
    allowlistSize < 2 ? allowlistSize + 1 : allowlistSize * 2 - 1;

  return estimateProofSizesWithFormula(nextAllowlistSize, result, proofSize);
}

function handleEstimateProofSizesWithFormula() {
  console.table(estimateProofSizesWithFormula(1, [], -1));
}

handleEstimateProofSizesWithFormula();

// Using tree construction: ***************************************************

function calculateProofLength(
  candyMachineKeypair: Keypair,
  allowlistBuyers: Array<MerkleAllowlistBuyerInfo>,
  leafCount: number
) {
  const buyersWithProof = constructMerkleAllowlist(
    candyMachineKeypair.publicKey,
    allowlistBuyers,
    leafCount
  );
  return buyersWithProof[0].buyersChunk[0].proof.length;
}

// @ts-ignore
// eslint-disable-next-line @typescript-eslint/no-unused-vars
function estimateProofSizesWithTreeConstruction() {
  console.log(
    `\nEstimating allowlist proof sizes up to a maximum allowlist size of ${MAX_ALLOWLIST_SIZE_TO_ESTIMATE}. This will take a moment...\n`
  );

  const candyMachineKeypair = Keypair.generate();
  const keypairs = generateKeypairArray(MAX_ALLOWLIST_SIZE_TO_ESTIMATE - 10000);
  const addresses = keypairs.map(({ publicKey }) => ({
    address: publicKey,
    amount: 1,
  }));

  const results: Array<ProofSizeResult> = [];

  let lastProofSize = -1;
  let currentAllowlistSize = 1;

  while (currentAllowlistSize < MAX_ALLOWLIST_SIZE_TO_ESTIMATE) {
    const proofSize = calculateProofLength(
      candyMachineKeypair,
      addresses,
      currentAllowlistSize
    );

    if (proofSize > lastProofSize) {
      results.push({
        ["Leaf count per tree"]: currentAllowlistSize,
        ["Proof length"]: proofSize,
        ["Proof size in bytes"]: proofSize * BYTES_PER_PROOF_ELEM,
      });
      lastProofSize = proofSize;
    }

    if (currentAllowlistSize === 1) {
      currentAllowlistSize++;
    } else {
      currentAllowlistSize = currentAllowlistSize * 2 - 1;
    }
  }

  console.table(results);
}

// Uncomment to estimate using merkle tree + proof construction.
// estimateProofSizesWithTreeConstruction();
