import {
  findAtaPda,
  Maybe,
} from "@formfunction-hq/formfunction-program-shared";
import { PublicKey } from "@solana/web3.js";
import CandyMachineAccount from "sdk/types/candy-machine/CandyMachineAccount";
import CandyMachineSplTokenAllowlistSettings from "sdk/types/candy-machine/CandyMachineSplTokenAllowlistSettings";
import SplTokenAllowlistMode from "sdk/types/candy-machine/SplTokenAllowlistMode";
import parseSplTokenAllowlistModeEnum from "sdk/utils/parseSplTokenAllowlistModeEnum";

export default async function getBuyerAllowlistTokenAccount(
  candyMachineAccount: CandyMachineAccount,
  buyer: PublicKey
): Promise<Maybe<PublicKey>> {
  const allowlistSettings = candyMachineAccount.data
    .splTokenAllowlistSettings as Maybe<CandyMachineSplTokenAllowlistSettings>;

  if (
    allowlistSettings != null &&
    parseSplTokenAllowlistModeEnum(allowlistSettings.mode) ===
      SplTokenAllowlistMode.BurnEveryTime
  ) {
    const [ata] = findAtaPda(buyer, allowlistSettings.mint);
    return ata;
  }

  return null;
}
