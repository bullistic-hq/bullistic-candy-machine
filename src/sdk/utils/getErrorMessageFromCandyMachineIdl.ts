import { Maybe } from "@formfunction-hq/formfunction-program-shared";
import { FORMFN_CANDY_MACHINE_IDL } from "sdk/idl";

export default function getErrorMessageFromCandyMachineIdl(
  errorCode: number
): Maybe<string> {
  const idlError = FORMFN_CANDY_MACHINE_IDL.errors.find(
    (e) => e.code === errorCode
  );
  return idlError?.msg ?? null;
}
