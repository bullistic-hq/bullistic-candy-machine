import { MaybeUndef } from "@bullistic-hq/bullistic-program-shared";
import KeypairObject from "tests/types/KeypairObject";

type SplAllowlistTestConfigJson = {
  allowlistMembers: Array<{ address: string; balance: string }>;
  keypairs: MaybeUndef<Array<KeypairObject>>;
  splTokenAllowlistMint: string;
};

export default SplAllowlistTestConfigJson;
