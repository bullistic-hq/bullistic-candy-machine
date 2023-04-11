import { assertUnreachable } from "@bullistic-hq/bullistic-program-shared";
import SplTokenAllowlistMode from "sdk/types/candy-machine/SplTokenAllowlistMode";
import SplTokenAllowlistSettingsModeAnchorIdl from "sdk/types/candy-machine/SplTokenAllowlistSettingsModeAnchorIdl";

export default function parseSplTokenAllowlistModeEnum(
  anchorData: SplTokenAllowlistSettingsModeAnchorIdl
): SplTokenAllowlistMode {
  const mode = Object.keys(anchorData)[0] as unknown as SplTokenAllowlistMode;

  switch (mode) {
    case SplTokenAllowlistMode.BurnEveryTime:
      return mode;
    case SplTokenAllowlistMode.NeverBurn:
      return mode;
    default:
      return assertUnreachable(mode);
  }
}
