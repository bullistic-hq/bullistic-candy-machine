import {
  arePublicKeysEqual,
  deserializeMerkleProof,
  findAtaPda,
  findEditionPda,
  findTokenMetadataPda,
  Maybe,
  TOKEN_METADATA_PROGRAM_ID,
} from "@bullistic-hq/bullistic-program-shared";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  AccountMeta,
  PublicKey,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  SYSVAR_RENT_PUBKEY,
  SYSVAR_SLOT_HASHES_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import BN from "bn.js";
import dayjs from "dayjs";
import { CandyMachineProgram } from "sdk/idl";
import findBuyerInfoAccountPda from "sdk/pdas/findBuyerInfoAccountPda";
import findCandyMachineCreatorPda from "sdk/pdas/findCandyMachineCreatorPda";
import BuyerWithAllowlistProofData from "sdk/types/BuyerWithAllowlistProofData";
import CandyMachineAccount from "sdk/types/candy-machine/CandyMachineAccount";
import SplTokenAllowlistMode from "sdk/types/candy-machine/SplTokenAllowlistMode";
import MintPhase from "sdk/types/MintPhase";
import getMintPhase from "sdk/utils/getMintPhase";
import parseSplTokenAllowlistModeEnum from "sdk/utils/parseSplTokenAllowlistModeEnum";

function getIxMerkleAllowlistProofData(
  buyerWithAllowlistProofData: Maybe<BuyerWithAllowlistProofData>
) {
  if (buyerWithAllowlistProofData == null) {
    return null;
  }

  const {
    amount,
    serializedProof: proof,
    rootIndexForProof,
  } = buyerWithAllowlistProofData;
  return {
    amount,
    proof: deserializeMerkleProof(proof).map((val) => [...val]),
    rootIndexForProof,
  };
}

async function getSplAllowlistSettingRemainingAccounts(
  candyMachineState: CandyMachineAccount,
  buyerAllowlistTokenAccount: Maybe<PublicKey>
): Promise<Array<AccountMeta>> {
  if (buyerAllowlistTokenAccount == null) {
    return [];
  }

  const { allowlistSaleStartTime, publicSaleEndTime, publicSaleStartTime } =
    candyMachineState.data;
  const mintPhase = getMintPhase({
    allowlistSaleStartTimeUnix:
      allowlistSaleStartTime == null
        ? null
        : dayjs.unix(allowlistSaleStartTime.toNumber()),
    publicSaleEndTimeUnix: dayjs.unix(publicSaleEndTime.toNumber()),
    publicSaleStartTimeUnix: dayjs.unix(publicSaleStartTime.toNumber()),
  });
  const { splTokenAllowlistSettings } = candyMachineState.data;
  if (splTokenAllowlistSettings == null || mintPhase !== MintPhase.Allowlist) {
    return [];
  }

  const { mint, mode } = splTokenAllowlistSettings;
  const buyerTokenAccount: AccountMeta = {
    isSigner: false,
    isWritable: true,
    pubkey: buyerAllowlistTokenAccount,
  };

  const remainingAccounts: Array<AccountMeta> = [buyerTokenAccount];

  if (
    parseSplTokenAllowlistModeEnum(mode) === SplTokenAllowlistMode.BurnEveryTime
  ) {
    const splTokenAllowlistMintAccount: AccountMeta = {
      isSigner: false,
      isWritable: true,
      pubkey: mint,
    };
    remainingAccounts.push(splTokenAllowlistMintAccount);
  }

  return remainingAccounts;
}

type Accounts = {
  botSignerAuthority: PublicKey;
  buyer: PublicKey;
  buyerAllowlistTokenAccount: Maybe<PublicKey>;
  candyMachine: PublicKey;
  mint: PublicKey;
};

type Args = {
  buyerWithAllowlistProofData: Maybe<BuyerWithAllowlistProofData>;
  expectedPrice: BN;
  program: CandyMachineProgram;
};

export default async function candyMachineMintNftIx(
  {
    botSignerAuthority,
    buyer,
    buyerAllowlistTokenAccount,
    candyMachine,
    mint,
  }: Accounts,
  { buyerWithAllowlistProofData, expectedPrice: expectedPrice, program }: Args
): Promise<TransactionInstruction> {
  const [buyerTokenAccount] = findAtaPda(buyer, mint);
  const [metadata] = findTokenMetadataPda(mint);
  const [masterEdition] = findEditionPda(mint);
  const [buyerInfoAccount, buyerInfoAccountBump] = findBuyerInfoAccountPda(
    candyMachine,
    buyer,
    program.programId
  );
  const [candyMachineCreator, candyMachineCreatorBump] =
    findCandyMachineCreatorPda(candyMachine, program.programId);

  const candyMachineState = (await program.account.candyMachine.fetch(
    candyMachine
  )) as unknown as CandyMachineAccount;

  const splTokenRemainingAccounts =
    await getSplAllowlistSettingRemainingAccounts(
      candyMachineState,
      buyerAllowlistTokenAccount
    );

  const ix = await program.methods
    .mintNft(
      candyMachineCreatorBump,
      buyerInfoAccountBump,
      getIxMerkleAllowlistProofData(buyerWithAllowlistProofData),
      expectedPrice
    )
    .accounts({
      ataProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      botSignerAuthority,
      buyer,
      buyerInfoAccount,
      buyerTokenAccount,
      candyMachine,
      candyMachineCreator,
      creatorAuthority: candyMachineState.creatorAuthority,
      instructionSysvarAccount: SYSVAR_INSTRUCTIONS_PUBKEY,
      masterEdition,
      metadata,
      mint,
      recentSlothashes: SYSVAR_SLOT_HASHES_PUBKEY,
      rent: SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      treasuryWallet: candyMachineState.treasuryWallet,
    })
    .remainingAccounts(splTokenRemainingAccounts)
    .instruction();

  // If bot protection measures are enabled for this candy machine, we want to
  // set the bot signer authority account as a signer.
  ix.keys = ix.keys.map((key) => {
    if (
      candyMachineState.data.botProtectionEnabled === true &&
      arePublicKeysEqual(key.pubkey, botSignerAuthority)
    ) {
      return { ...key, isSigner: true };
    }

    return key;
  });

  return ix;
}
