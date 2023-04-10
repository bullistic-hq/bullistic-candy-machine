import {
  PdaResult,
  TOKEN_METADATA_PROGRAM_ID,
} from "@formfunction-hq/formfunction-program-shared";
import { PublicKey } from "@solana/web3.js";

export default function findCandyMachineCollectionAuthorityPda(
  mint: PublicKey,
  authority: PublicKey
): PdaResult {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
      Buffer.from("collection_authority"),
      authority.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );
}
