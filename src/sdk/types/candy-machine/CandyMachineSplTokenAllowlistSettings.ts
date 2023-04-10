import { PublicKey } from "@solana/web3.js";
import SplTokenAllowlistSettingsModeAnchorIdl from "sdk/types/candy-machine/SplTokenAllowlistSettingsModeAnchorIdl";

// For some reason the Anchor types are not inferred correctly for this.
// Note: Keep in sync with program data.
type CandyMachineSplTokenAllowlistSettings = {
  mint: PublicKey;
  mode: SplTokenAllowlistSettingsModeAnchorIdl;
};

export default CandyMachineSplTokenAllowlistSettings;
