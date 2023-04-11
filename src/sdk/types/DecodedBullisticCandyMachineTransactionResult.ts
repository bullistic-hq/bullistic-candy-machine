/**
 * NOTE: This is an auto-generated file. Don't edit it directly.
 */
import {
  DecodedInstructionAccount,
  GenericDecodedTransaction,
} from "@bullistic-hq/bullistic-program-shared";
import { IDL as BULLISTIC_CANDY_MACHINE_IDL } from "sdk/idl/BullisticCandyMachine";
import BullisticCandyMachineInstructionName from "sdk/types/BullisticCandyMachineInstructionName";

const identity = <T>(val: T): T => val;

const ixMap = BULLISTIC_CANDY_MACHINE_IDL.instructionsMap ?? {};

const AddConfigLinesAccounts = (ixMap.addConfigLines ?? []).map(identity);

const AppendMerkleAllowlistRootsAccounts = (
  ixMap.appendMerkleAllowlistRoots ?? []
).map(identity);

const ClearMerkleAllowlistRootsAccounts = (
  ixMap.clearMerkleAllowlistRoots ?? []
).map(identity);

const InitializeCandyMachineAccounts = (ixMap.initializeCandyMachine ?? []).map(
  identity
);

const MintNftAccounts = (ixMap.mintNft ?? []).map(identity);

const RemoveCollectionAccounts = (ixMap.removeCollection ?? []).map(identity);

const RemoveFreezeAccounts = (ixMap.removeFreeze ?? []).map(identity);

const SetCollectionAccounts = (ixMap.setCollection ?? []).map(identity);

const SetCollectionDuringMintAccounts = (
  ixMap.setCollectionDuringMint ?? []
).map(identity);

const SetFreezeAccounts = (ixMap.setFreeze ?? []).map(identity);

const ThawNftAccounts = (ixMap.thawNft ?? []).map(identity);

const UnlockFundsAccounts = (ixMap.unlockFunds ?? []).map(identity);

const UpdateAuthorityAccounts = (ixMap.updateAuthority ?? []).map(identity);

const UpdateCandyMachineAccounts = (ixMap.updateCandyMachine ?? []).map(
  identity
);

const WithdrawFundsAccounts = (ixMap.withdrawFunds ?? []).map(identity);

type DecodedBullisticCandyMachineTransactionResult = {
  addConfigLines?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof AddConfigLinesAccounts[0]]: DecodedInstructionAccount;
    };
  };
  appendMerkleAllowlistRoots?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof AppendMerkleAllowlistRootsAccounts[0]]: DecodedInstructionAccount;
    };
  };
  clearMerkleAllowlistRoots?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof ClearMerkleAllowlistRootsAccounts[0]]: DecodedInstructionAccount;
    };
  };
  initializeCandyMachine?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof InitializeCandyMachineAccounts[0]]: DecodedInstructionAccount;
    };
  };
  mintNft?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof MintNftAccounts[0]]: DecodedInstructionAccount;
    };
  };
  removeCollection?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof RemoveCollectionAccounts[0]]: DecodedInstructionAccount;
    };
  };
  removeFreeze?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof RemoveFreezeAccounts[0]]: DecodedInstructionAccount;
    };
  };
  setCollection?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof SetCollectionAccounts[0]]: DecodedInstructionAccount;
    };
  };
  setCollectionDuringMint?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof SetCollectionDuringMintAccounts[0]]: DecodedInstructionAccount;
    };
  };
  setFreeze?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof SetFreezeAccounts[0]]: DecodedInstructionAccount;
    };
  };
  thawNft?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof ThawNftAccounts[0]]: DecodedInstructionAccount;
    };
  };
  unlockFunds?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof UnlockFundsAccounts[0]]: DecodedInstructionAccount;
    };
  };
  updateAuthority?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof UpdateAuthorityAccounts[0]]: DecodedInstructionAccount;
    };
  };
  updateCandyMachine?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof UpdateCandyMachineAccounts[0]]: DecodedInstructionAccount;
    };
  };
  withdrawFunds?: GenericDecodedTransaction<BullisticCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof WithdrawFundsAccounts[0]]: DecodedInstructionAccount;
    };
  };
};

export default DecodedBullisticCandyMachineTransactionResult;
