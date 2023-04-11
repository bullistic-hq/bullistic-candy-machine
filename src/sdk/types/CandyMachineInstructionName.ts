import { BULLISTIC_CANDY_MACHINE_IDL } from "sdk/idl";

const INSTRUCTION_NAMES = BULLISTIC_CANDY_MACHINE_IDL.instructions.map(
  (ix) => ix.name
);

type CandyMachineInstructionName = typeof INSTRUCTION_NAMES[0];

export default CandyMachineInstructionName;
