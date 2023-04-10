import {
  decodeTransactionUsingProgramIdl,
  Maybe,
} from "@formfunction-hq/formfunction-program-shared";
import { ParsedTransactionWithMeta, PublicKey } from "@solana/web3.js";
import { FORMFN_CANDY_MACHINE_IDL } from "sdk/idl";
import DecodedFormfnCandyMachineTransactionResult from "sdk/types/DecodedFormfnCandyMachineTransactionResult";

export default function decodeCandyMachineTransaction(
  programId: PublicKey,
  parsedTransaction: ParsedTransactionWithMeta
): Maybe<DecodedFormfnCandyMachineTransactionResult> {
  for (const idl of [FORMFN_CANDY_MACHINE_IDL]) {
    const result =
      decodeTransactionUsingProgramIdl<DecodedFormfnCandyMachineTransactionResult>(
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
