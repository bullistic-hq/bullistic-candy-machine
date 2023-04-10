import { PublicKey } from "@solana/web3.js";

type CandyMachineProgramIds = {
  botSignerAuthority: PublicKey;
  candyMachineProgramId: PublicKey;
};

export const LOCALNET_PROGRAM_IDS: CandyMachineProgramIds = {
  botSignerAuthority: new PublicKey(
    "antiDV8bRvF4XTeRqmyHV1jpHD4Lvz7gKBKBBRQb8ir"
  ),
  candyMachineProgramId: new PublicKey(
    "gachaC2NGh63y4ogLK8xHLeB5ZFZ8ypDLXEQyKNm8sy"
  ),
};

export const TESTNET_PROGRAM_IDS: CandyMachineProgramIds = {
  botSignerAuthority: new PublicKey(
    "antiDV8bRvF4XTeRqmyHV1jpHD4Lvz7gKBKBBRQb8ir"
  ),
  candyMachineProgramId: new PublicKey(
    "gachaC2NGh63y4ogLK8xHLeB5ZFZ8ypDLXEQyKNm8sy"
  ),
};

export const DEVNET_PROGRAM_IDS: CandyMachineProgramIds = {
  botSignerAuthority: new PublicKey(
    "antiDV8bRvF4XTeRqmyHV1jpHD4Lvz7gKBKBBRQb8ir"
  ),
  candyMachineProgramId: new PublicKey(
    "gachaC2NGh63y4ogLK8xHLeB5ZFZ8ypDLXEQyKNm8sy"
  ),
};

export const MAINNET_PROGRAM_IDS: CandyMachineProgramIds = {
  botSignerAuthority: new PublicKey(
    "antiScHGm8NAqfpdFNYbv3c9ntY6xksvvTN3B9cDf5Y"
  ),
  candyMachineProgramId: new PublicKey(
    "gachaC2NGh63y4ogLK8xHLeB5ZFZ8ypDLXEQyKNm8sy"
  ),
};
