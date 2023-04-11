import { PdaResult } from "@bullistic-hq/bullistic-program-shared";
import { PublicKey } from "@solana/web3.js";

export default function findCandyMachineCollectionPda(
  candyMachine: PublicKey,
  candyMachineProgramId: PublicKey
): PdaResult {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("collection"), candyMachine.toBuffer()],
    candyMachineProgramId
  );
}
