import { MaybeUndef } from "@bullistic-hq/bullistic-program-shared";
import dayjs, { Dayjs } from "dayjs";
import MintPhase from "sdk/types/MintPhase";

// Mirrors logic in program CandyMachine::get_mint_phase.
// Note: All the original cm field are in unix time in seconds.
export default function getMintPhase({
  allowlistSaleStartTimeUnix,
  publicSaleEndTimeUnix,
  publicSaleStartTimeUnix,
}: {
  allowlistSaleStartTimeUnix: MaybeUndef<Dayjs>;
  publicSaleEndTimeUnix: Dayjs;
  publicSaleStartTimeUnix: Dayjs;
}): MintPhase {
  const now = dayjs();
  if (now.isAfter(publicSaleEndTimeUnix)) {
    return MintPhase.Expired;
  }

  if (now.isAfter(publicSaleStartTimeUnix)) {
    return MintPhase.Public;
  }

  if (allowlistSaleStartTimeUnix != null) {
    if (now.isAfter(allowlistSaleStartTimeUnix)) {
      return MintPhase.Allowlist;
    }
  }

  return MintPhase.Premint;
}
