import {
  findEditionPda,
  findTokenMetadataPda,
  TOKEN_METADATA_PROGRAM_ID,
} from "@formfunction-hq/formfunction-program-shared";
import {
  PublicKey,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import { CandyMachineProgram } from "sdk/idl";
import findCandyMachineCollectionAuthorityPda from "sdk/pdas/findCandyMachineCollectionAuthorityPda";
import findCandyMachineCollectionPda from "sdk/pdas/findCandyMachineCollectionPda";

type Accounts = {
  buyer: PublicKey;
  candyMachine: PublicKey;
  mint: PublicKey;
};

type Args = {
  program: CandyMachineProgram;
};

export default async function candyMachineSetCollectionDuringMintIx(
  { buyer, candyMachine, mint }: Accounts,
  { program }: Args
): Promise<TransactionInstruction> {
  const [collectionPda] = findCandyMachineCollectionPda(
    candyMachine,
    program.programId
  );
  const collectionPdaAccount = await program.account.collectionPda.fetch(
    collectionPda
  );
  const [metadata] = findTokenMetadataPda(mint);
  const [collectionPdaMetadata] = findTokenMetadataPda(
    collectionPdaAccount.mint
  );
  const [collectionAuthorityPubkey] = findCandyMachineCollectionAuthorityPda(
    collectionPdaAccount.mint,
    collectionPda
  );
  const [collectionMasterEdition] = findEditionPda(collectionPdaAccount.mint);

  const candyMachineState = await program.account.candyMachine.fetch(
    candyMachine
  );

  return program.methods
    .setCollectionDuringMint()
    .accounts({
      buyer,
      candyMachine,
      collectionAuthorityRecord: collectionAuthorityPubkey,
      collectionMasterEdition,
      collectionMetadata: collectionPdaMetadata,
      collectionMint: collectionPdaAccount.mint,
      collectionPda,
      creatorAuthority: candyMachineState.creatorAuthority,
      instructionSysvarAccount: SYSVAR_INSTRUCTIONS_PUBKEY,
      metadata,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .instruction();
}
