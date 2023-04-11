import modifyProgramIdlScript from "@bullistic-hq/bullistic-program-shared/dist/scripts/modifyProgramIdlScript";

modifyProgramIdlScript({
  decodedTransactionResultTypeFilePath:
    "src/sdk/types/DecodedBullisticCandyMachineTransactionResult.ts",
  idlFilePath: "src/sdk/idl/BullisticCandyMachine.ts",
  programName: "BullisticCandyMachine",
});
