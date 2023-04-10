import { PdaResult } from "@formfunction-hq/formfunction-program-shared";
import { PublicKey } from "@solana/web3.js";

export default function findCandyMachineCreatorPda(
  candyMachine: PublicKey,
  candyMachineProgramId: PublicKey
): PdaResult {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("candy_machine"), candyMachine.toBuffer()],
    candyMachineProgramId
  );
}
