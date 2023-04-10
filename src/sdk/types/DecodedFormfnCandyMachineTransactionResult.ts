/**
 * NOTE: This is an auto-generated file. Don't edit it directly.
 */
import {
  DecodedInstructionAccount,
  GenericDecodedTransaction,
} from "@formfunction-hq/formfunction-program-shared";
import { IDL as FORMFN_CANDY_MACHINE_IDL } from "sdk/idl/FormfnCandyMachine";
import FormfnCandyMachineInstructionName from "sdk/types/FormfnCandyMachineInstructionName";

const identity = <T>(val: T): T => val;

const ixMap = FORMFN_CANDY_MACHINE_IDL.instructionsMap ?? {};

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

type DecodedFormfnCandyMachineTransactionResult = {
  addConfigLines?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof AddConfigLinesAccounts[0]]: DecodedInstructionAccount;
    };
  };
  appendMerkleAllowlistRoots?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof AppendMerkleAllowlistRootsAccounts[0]]: DecodedInstructionAccount;
    };
  };
  clearMerkleAllowlistRoots?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof ClearMerkleAllowlistRootsAccounts[0]]: DecodedInstructionAccount;
    };
  };
  initializeCandyMachine?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof InitializeCandyMachineAccounts[0]]: DecodedInstructionAccount;
    };
  };
  mintNft?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof MintNftAccounts[0]]: DecodedInstructionAccount;
    };
  };
  removeCollection?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof RemoveCollectionAccounts[0]]: DecodedInstructionAccount;
    };
  };
  removeFreeze?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof RemoveFreezeAccounts[0]]: DecodedInstructionAccount;
    };
  };
  setCollection?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof SetCollectionAccounts[0]]: DecodedInstructionAccount;
    };
  };
  setCollectionDuringMint?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof SetCollectionDuringMintAccounts[0]]: DecodedInstructionAccount;
    };
  };
  setFreeze?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof SetFreezeAccounts[0]]: DecodedInstructionAccount;
    };
  };
  thawNft?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof ThawNftAccounts[0]]: DecodedInstructionAccount;
    };
  };
  unlockFunds?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof UnlockFundsAccounts[0]]: DecodedInstructionAccount;
    };
  };
  updateAuthority?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof UpdateAuthorityAccounts[0]]: DecodedInstructionAccount;
    };
  };
  updateCandyMachine?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof UpdateCandyMachineAccounts[0]]: DecodedInstructionAccount;
    };
  };
  withdrawFunds?: GenericDecodedTransaction<FormfnCandyMachineInstructionName> & {
    accountsMap: {
      [Key in typeof WithdrawFundsAccounts[0]]: DecodedInstructionAccount;
    };
  };
};

export default DecodedFormfnCandyMachineTransactionResult;
