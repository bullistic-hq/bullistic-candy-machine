import {
  decodeTransactionUsingProgramIdl,
  Maybe,
} from "@bullistic-hq/bullistic-program-shared";
import { ParsedTransactionWithMeta, PublicKey } from "@solana/web3.js";
import { BULLISTIC_CANDY_MACHINE_IDL } from "sdk/idl";
import DecodedBullisticCandyMachineTransactionResult from "sdk/types/DecodedBullisticCandyMachineTransactionResult";

export default function decodeCandyMachineTransaction(
  programId: PublicKey,
  parsedTransaction: ParsedTransactionWithMeta
): Maybe<DecodedBullisticCandyMachineTransactionResult> {
  for (const idl of [BULLISTIC_CANDY_MACHINE_IDL]) {
    const result =
      decodeTransactionUsingProgramIdl<DecodedBullisticCandyMachineTransactionResult>(
        idl,
        programId,
        parsedTransaction
      );
    if (result != null) {
      return result;
    }
  }

  return null;
}
