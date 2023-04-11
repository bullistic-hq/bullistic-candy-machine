import { Maybe } from "@bullistic-hq/bullistic-program-shared";
import { CandyMachineProgram } from "sdk/idl";
import CandyMachineSplTokenAllowlistSettings from "sdk/types/candy-machine/CandyMachineSplTokenAllowlistSettings";

type CandyMachineIdlAccount = Awaited<
  ReturnType<CandyMachineProgram["account"]["candyMachine"]["fetch"]>
>;

type CandyMachineAccountData = Omit<
  CandyMachineIdlAccount["data"],
  "splTokenAllowlistSettings"
>;

type CandyMachineAccount = Omit<CandyMachineIdlAccount, "data"> & {
  data: CandyMachineAccountData & {
    // Note: Keep in sync with program spl_token_allowlist_settings field.
    splTokenAllowlistSettings: Maybe<CandyMachineSplTokenAllowlistSettings>;
  };
};

export default CandyMachineAccount;
