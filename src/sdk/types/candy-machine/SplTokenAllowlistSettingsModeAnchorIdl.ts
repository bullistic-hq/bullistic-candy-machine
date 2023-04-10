import SplTokenAllowlistMode from "sdk/types/candy-machine/SplTokenAllowlistMode";

// The Anchor types look like this: { burnEveryTime: {} }.
// Note: Keep in sync with program data.
type SplTokenAllowlistSettingsModeAnchorIdl =
  | { [SplTokenAllowlistMode.BurnEveryTime]: never }
  | { [SplTokenAllowlistMode.NeverBurn]: never };

export default SplTokenAllowlistSettingsModeAnchorIdl;
