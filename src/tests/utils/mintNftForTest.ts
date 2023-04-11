import {
  ANTI_BOT_DEV_AUTHORITY_KEYPAIR,
  Maybe,
} from "@bullistic-hq/bullistic-program-shared";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import BullisticCandyMachineSdk from "sdk/BullisticCandyMachineSdk";
import BuyerWithAllowlistProofData from "sdk/types/BuyerWithAllowlistProofData";
import CandyMachineAccount from "sdk/types/candy-machine/CandyMachineAccount";
import getBuyerAllowlistTokenAccount from "tests/utils/getBuyerAllowlistTokenAccount";
import sendTransactionForTest from "tests/utils/sendTransactionForTest";

export default async function mintNftForTest({
  allowlistType,
  buyer,
  buyerWithAllowlistProofData,
  candyMachine,
  candyMachineSdk,
  candyMachineState,
  connection,
}: {
  allowlistType: "merkle" | "SPL";
  buyer: Keypair;
  buyerWithAllowlistProofData: Maybe<BuyerWithAllowlistProofData>;
  candyMachine: PublicKey;
  candyMachineSdk: BullisticCandyMachineSdk;
  candyMachineState: CandyMachineAccount;
  connection: Connection;
}): Promise<{ mint: PublicKey; txid: string }> {
  const mintKeypair = Keypair.generate();
  const mint = mintKeypair.publicKey;

  const buyerAllowlistTokenAccount = await getBuyerAllowlistTokenAccount(
    candyMachineState,
    buyer.publicKey
  );

  const extraLogMsg =
    allowlistType === "merkle"
      ? ` and merkle root index ${buyerWithAllowlistProofData?.rootIndexForProof}`
      : "";
  console.log(
    `Minting NFT for buyer ${buyer.publicKey.toString()} using ${allowlistType} allowlist${extraLogMsg}.`
  );

  const mintTx = await candyMachineSdk.mintNft(
    {
      buyer: buyer.publicKey,
      buyerAllowlistTokenAccount,
      candyMachine,
      mint,
    },
    {
      buyerWithAllowlistProofData,
    }
  );

  const signers = [buyer, mintKeypair];
  if (candyMachineState.data.botProtectionEnabled === true) {
    signers.push(ANTI_BOT_DEV_AUTHORITY_KEYPAIR);
  }

  const txid = await sendTransactionForTest(connection, mintTx, signers, {
    commitment: "confirmed",
    skipPreflight: true,
  });

  return { mint, txid };
}
