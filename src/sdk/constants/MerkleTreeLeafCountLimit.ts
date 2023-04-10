/**
 * This limit determines how many allowlist buyer addresses are used for
 * each merkle tree and by extension the size of each proof provided when
 * a buyer is minting.
 *
 * As such, the actual size limit depends on the mint ix size and how large
 * a proof can fit in that ix.
 *
 * For now, going with 200 which is near the limit for Gumdrop.
 *
 * TODO[@]: Figure out a more specific limit once everything is working.
 */
const MERKLE_TREE_LEAF_COUNT_LIMIT = 200;

export default MERKLE_TREE_LEAF_COUNT_LIMIT;
