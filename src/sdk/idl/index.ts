import { Program } from "@project-serum/anchor";
import {
  BullisticCandyMachine,
  IDL as BULLISTIC_CANDY_MACHINE_IDL,
} from "sdk/idl/BullisticCandyMachine";

export { BULLISTIC_CANDY_MACHINE_IDL };

export type CandyMachineProgram = Program<BullisticCandyMachine>;
