import { Maybe } from "@bullistic-hq/bullistic-program-shared";
import { BULLISTIC_CANDY_MACHINE_IDL } from "sdk/idl";

export default function getErrorMessageFromCandyMachineIdl(
  errorCode: number
): Maybe<string> {
  const idlError = BULLISTIC_CANDY_MACHINE_IDL.errors.find(
    (e) => e.code === errorCode
  );
  return idlError?.msg ?? null;
}
