import { MaybeUndef } from "@bullistic-hq/bullistic-program-shared";
import { Keypair, PublicKey } from "@solana/web3.js";

type SplAllowlistTestConfig = {
  allowlistMembers: Array<{ address: PublicKey; balance: string }>;
  keypairs: MaybeUndef<Array<Keypair>>;
  splTokenAllowlistMint: PublicKey;
};

export default SplAllowlistTestConfig;
