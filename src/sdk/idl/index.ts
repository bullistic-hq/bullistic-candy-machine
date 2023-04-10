import { Program } from "@project-serum/anchor";
import {
  FormfnCandyMachine,
  IDL as FORMFN_CANDY_MACHINE_IDL,
} from "sdk/idl/FormfnCandyMachine";

export { FORMFN_CANDY_MACHINE_IDL };

export type CandyMachineProgram = Program<FormfnCandyMachine>;
