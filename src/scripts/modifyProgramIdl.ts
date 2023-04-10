import modifyProgramIdlScript from "@formfunction-hq/formfunction-program-shared/dist/scripts/modifyProgramIdlScript";

modifyProgramIdlScript({
  decodedTransactionResultTypeFilePath:
    "src/sdk/types/DecodedFormfnCandyMachineTransactionResult.ts",
  idlFilePath: "src/sdk/idl/FormfnCandyMachine.ts",
  programName: "FormfnCandyMachine",
});
