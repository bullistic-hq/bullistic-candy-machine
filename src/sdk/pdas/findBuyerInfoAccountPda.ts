import { PdaResult } from "@bullistic-hq/bullistic-program-shared";
import { PublicKey } from "@solana/web3.js";

export default function findBuyerInfoAccountPda(
  candyMachine: PublicKey,
  buyer: PublicKey,
  candyMachineProgramId: PublicKey
): PdaResult {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("buyer_info_account"),
      candyMachine.toBuffer(),
      buyer.toBuffer(),
    ],
    candyMachineProgramId
  );
}
